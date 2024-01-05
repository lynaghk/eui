(ns frontend.main
  (:require-macros [lonocloud.synthread :as ->])
  (:require [frontend.macros :refer [p pp timeout interval]]
            clojure.string
            [rum.core :as rum]
            [cljs.core.match :refer-macros [match]]
            [fipp.edn :refer [pprint]]))


(declare trigger!)
(declare update-app)

(def $app-container
  (.getElementById js/document "app-container"))

(defn json->clj
  [json]
  (js->clj (.parse js/JSON json) :keywordize-keys true))



;; Serde json data representations: https://serde.rs/json.html for

;; struct W {
;;     a: i32,
;;     b: i32,
;; }
;; let w = W { a: 0, b: 0 }; // Represented as `{"a":0,"b":0}`

;; struct X(i32, i32);
;; let x = X(0, 0); // Represented as `[0,0]`

;; struct Y(i32);
;; let y = Y(0); // Represented as just the inner value `0`

;; struct Z;
;; let z = Z; // Represented as `null`

;; enum E {
;;     W { a: i32, b: i32 },
;;     X(i32, i32),
;;     Y(i32),
;;     Z,
;; }
;; let w = E::W { a: 0, b: 0 }; // Represented as `{"W":{"a":0,"b":0}}`
;; let x = E::X(0, 0);          // Represented as `{"X":[0,0]}`
;; let y = E::Y(0);             // Represented as `{"Y":0}`
;; let z = E::Z;                // Represented as `"Z"`

(defn default
  [type]
  (let [{:strs [name ty]} (type "NamedType" type)]
    (match ty
      {"Enum" variants} (let [v (first variants)]
                          {(v "name") (default v)})

      "UnitVariant" name

      {"TupleVariant" [t]} {name (default t)}

      {"TupleVariant" tuples} {name (mapv default tuples)}

      {"Struct" fields} (into {} (for [f fields]
                                   [(f "name") (default f)]))

      "U8" 0

      :else (do
              (pp "No default for type")
              (pp type)
              nil))))


(rum/defcs *render <
  (rum/local nil)
  {:did-mount (fn [state]
                (let [comp (:rum/react-component state)]
                  (-> state
                      (->/aside _
                        (add-watch (:rum/local state) :trigger-on-change (fn [_ _ old new]
                                                                           ;;TODO: this is a hack to make sure the watcher always invokes the latest `on-change` callback
                                                                           (let [{[_ _ on-change] :rum/args} @(rum/state comp)]
                                                                             (on-change new))))))))}

  [{!local :rum/local :as state}
   type
   value
   on-change]

  (let [{:strs [name ty]} (type "NamedType" type)
        value (or @!local (default type))]
    (match ty
      {"Enum" variants} [:.enum
                         [:span name]
                         [:ol
                          (for [v variants]
                            (let [selected? (or (= value (v "name"))
                                                (contains? value (v "name")))]
                              [:li {:data-is-selected selected?}
                               (*render v
                                        value
                                        #(reset! !local %))]))]]

      "UnitVariant" [:.variant
                     [:button {:on-click #(on-change name)}
                      name]]

      {"TupleVariant" [t]} [:.variant
                            [:button {:on-click #(reset! !local value)}
                             name]
                            (*render t
                                     (get value name)
                                     #(reset! !local (assoc value name %)))]

      {"TupleVariant" tuples} [:.variant
                               [:button {:on-click #(reset! !local value)}
                                name]
                               [:ol.tuples
                                (for [[idx t] (map-indexed vector tuples)]
                                  [:li
                                   (*render t
                                            (get-in value [name idx])
                                            #(reset! !local (assoc-in value [name idx] %)))])]]

      {"Struct" fields} [:.struct
                         [:span name]
                         (for [f fields]
                           [:.field
                            (*render f
                                     (get value (f "name"))
                                     #(reset! !local (assoc value (f "name") %)))])]

      "U8" [:label name
            [:input {:on-change (fn [e] (reset! !local (js/parseInt (.-value (.-target e)))))
                     :type :range
                     :value value
                     :min 0 :max 255}]
            [:input {:on-change (fn [e]
                                  (let [s (.-value (.-target e))
                                        x (js/parseFloat s)]
                                    (reset! !local (if (NaN? x) "" x))))
                     :value value}]]



      :else (do
              (pp "Unable to render type")
              (pp ty)
              nil))))


(rum/defcs *app <
  {:did-mount (fn [state]
                (-> state
                    (->/aside {[trigger! !app] :rum/args}
                      (add-watch !app :re-render-app (fn [_ _ old new]
                                                       (when-not (= old new)
                                                         (rum/request-render (:rum/react-component state))))))))

   :did-catch (fn [state error info]
                (p error)
                (assoc state ::error error))}

  (rum/local nil)

  [{!value :rum/local :as state} trigger! !app]

  (let [{:keys [schema value] :as app} @!app]
    [:.app
     [:.connected

      (*render schema
               value
               (fn [x] (reset! !value x)))

      ;;[:pre (with-out-str (pprint schema))]
      (let [v @!value]
        [:section [:h2 "Output"]
         [:pre (with-out-str (pprint v))]
         [:button {:on-click #(-> (js/fetch "/cmd" #js {:credentials "include"
                                                        :method "POST"
                                                        :headers #js {"Content-Type" "application/json"}
                                                        :body (.stringify js/JSON (clj->js v))})
                                  (.then (fn [response]
                                           (if (not (.-ok response))
                                             (.then (.text response) (fn [t]
                                                                       (.error js/console t)
                                                                       (throw (js/Error. (str "Bad response: " (.-status response))))))
                                             (.text response))))
                                  ;;(.then on-success)
                                  (.catch (fn [e]
                                            (.error js/console e))))}
          "Send"
          ]
         ])]]))


(defn update-app
  [app event]

  (match event

    :else
    (do
      (js/console.warn "unknown event")
      (pp event)
      app)))



(defn start!
  [$container opts]
  (let [!app (atom opts)]

    (letfn [;;this fn called by livereloading
            (full-render! []
              (rum/mount (*app trigger! !app) $app-container))

            (trigger!
              ([event-name event]
               (trigger! (assoc event :event event-name)))

              ([event]
               (try
                 (let [app @!app
                       new-app (update-app app event)]

                   (when (and new-app
                              (not (identical? app new-app)))

                     ;;updating atom will trigger re-render of app
                     (reset! !app new-app)))

                 (catch js/Error e
                   (p (.-message e))
                   (p (.-stack e))
                   nil))))]


      (defn ^:dev/after-load after-load!
        []
        (full-render!))

      ;;initial render
      (full-render!))))


(defn main
  []
  (start! $app-container {:schema (js->clj js/SCHEMA_COMMAND)}))
