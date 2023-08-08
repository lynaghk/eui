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


(rum/defcs *render < (rum/local nil)
  [{!local :rum/local :as state}
   {:strs [name ty]}
   value
   on-change]

  ;;See https://serde.rs/json.html for default serde json data representations

  (let [value (or @!local value)]
    (match ty
      {"Enum" variants} [:.enum
                         [:span name]
                         [:ol
                          (for [v variants]
                            (let [selected? (or (= value (v "name"))
                                                (contains? value (v "name")))]
                              [:li {:data-is-selected selected?}
                               selected?
                               (*render v
                                        nil
                                        (fn [x]
                                          (reset! !local x)
                                          (on-change @!local)))]))]]

      "UnitVariant" [:.variant
                     [:button {:on-click #(on-change name)}
                      name]]

      {"TupleVariant" [t]} (let [value (or value {name nil})]
                             [:.variant
                              [:button {:on-click #(on-change value)}
                               name]
                              (*render t
                                       (get value name)
                                       (fn [x]
                                         (reset! !local (assoc value name x))
                                         (on-change @!local)))])

      {"TupleVariant" tuples} (let [value (or value {name (vec (repeat (count tuples) nil))})]
                                [:.variant
                                 [:button {:on-click #(on-change value)}
                                  name]
                                 [:ol.tuples
                                  (for [[idx t] (map-indexed vector tuples)]
                                    [:li
                                     (*render t
                                              (get-in value [name idx])
                                              (fn [x]
                                                (reset! !local (assoc-in value [name idx] x))
                                                (on-change @!local)))])]])

      {"Struct" fields} [:.struct
                         [:span name]
                         (for [[idx f] (map-indexed vector fields)]
                           [:.field
                            (*render f
                                     (get value(f "name"))
                                     (fn [x]
                                       (reset! !local (assoc value (f "name") x))
                                       (on-change @!local)))])]

      {"name" "u8" "ty" "U8"} [:label name
                               [:input {:on-change (fn [e] (on-change (js/parseInt (.-value (.-target e)))))
                                        :type :range
                                        :value value
                                        :min 0 :max 255}]
                               [:input {:on-change (fn [e] (on-change (js/parseInt (.-value (.-target e)))))
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

  (let [{:keys [schema value fns] :as app} @!app]
    [:.app
     [:pre (with-out-str (pprint schema))]
     [:pre (with-out-str (pprint value))]

     [:section [:h2 "Output"]
      [:pre (with-out-str (pprint @!value))]
      (when-let [valid? (aget fns "eui_is_valid")]
        [:span
         "Valid? " (when-let [v @!value]
                     (str (valid? (js/JSON.stringify (clj->js v)))))])]

     [:section [:h2 "Picker"]
      (*render schema
               value
               (fn [x] (reset! !value x)))]

     ;; (*render {:name "Test" :ty "U8"}
     ;;          (fn [x] (p x))
     ;;          value)
     ]))


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

      ;; this fn called by rust
      (aset js/window "eui_invoke" (fn [s]
                                     (let [[schema value] (js->clj (.parse js/JSON s))]
                                       (swap! !app assoc :schema schema :value value))))

      ;; expose wasm fns in js
      (.addEventListener js/document "eui-wasm-loaded" (fn [e]
                                                         (swap! !app assoc :fns (aget e "detail" "bindings"))))

      ;;initial render
      (full-render!))))


(defn main
  []
  (start! $app-container {}))
