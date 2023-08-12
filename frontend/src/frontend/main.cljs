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



;; See https://serde.rs/json.html for default serde json data representations

(defn default
  [{:strs [name ty] :as type}]
  (match ty
    {"Enum" variants} (let [v (first variants)]
                        {(v "name") (default v)})

    "UnitVariant" name

    {"TupleVariant" [t]} {name (default t)}

    {"TupleVariant" tuples} {name (mapv default tuples)}

    {"Struct" fields} (into {} (for [f fields]
                                 [(f "name") (default f)]))

    {"name" "u8" "ty" "U8"} 0

    :else (do
            (pp "No default for type")
            (pp type)
            nil)))


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
   {:strs [name ty] :as type}
   value
   on-change]

  (let [value (or @!local (default type))]
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

      {"name" "u8" "ty" "U8"} [:label name
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

  (let [{:keys [schema value fns port] :as app} @!app]
    [:.app

     (cond

       (nil? port)
       [:button {:on-click (fn [e]
                             (-> (.-serial js/navigator)
                                 (.requestPort #js {})
                                 (.then (fn [port]
                                          (-> (.open port #js {:baudRate 115200})
                                              (.then (fn []
                                                       (.addEventListener port "disconnect" #(trigger! :port-closed {:port port}))
                                                       (trigger! :port-opened {:port (.getWriter (.-writable port))}))))))
                                 (.catch (fn [err]
                                           (p err)))))}
        "Connect"]


       (not (nil? port))
       [:.connected

        [:pre (with-out-str (pprint schema))]

        [:section [:h2 "Output"]
         [:pre (with-out-str (pprint @!value))]
         (when-let [serialize (some-> fns (aget "eui_serialize"))]
           (when-let [v @!value]
             (let [bytes (serialize (js/JSON.stringify (clj->js v)))
                   valid? (not (nil? bytes))]

               (when valid?
                 (.write port bytes))

               [:span "Valid? " (str valid?)])))]

        [:section [:h2 "Picker"]
         (*render schema
                  value
                  (fn [x] (reset! !value x)))]])






     ;; (*render {:name "Test" :ty "U8"}
     ;;          (fn [x] (p x))
     ;;          value)
     ]))


(defn update-app
  [app event]

  (match event

    {:event :port-opened :port port}
    (do
      (when-let [p (:port app)]
        (.close p))
      (assoc app :port port))

    {:event :port-closed :port port}
    (-> app
        (->/when (= port (:port app))
          (dissoc :port)))

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
      ;; (aset js/window "eui_invoke" (fn [s]
      ;;                                (swap! !app assoc :schema s)))

      ;; (.then (.getPorts (.-serial js/navigator)) (fn [ports]
      ;;                                              (trigger! :new-ports {:ports ports})))

      ;; expose wasm fns in js
      (.addEventListener js/document "eui-wasm-loaded" (fn [e]
                                                         (swap! !app assoc
                                                                :fns (aget e "detail" "bindings")
                                                                :schema (-> ((aget e "detail" "bindings" "eui_schema"))
                                                                            js/JSON.parse
                                                                            js->clj))))



      ;;initial render
      (full-render!))))


(defn main
  []
  (start! $app-container {}))
