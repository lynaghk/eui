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
  [{!value :rum/local :as state}
   {:strs [name ty]}
   on-change
   value]

  (match ty
    {"Enum" variants} [:label name]


    "UnitVariant" [:label name]

    "U8" [:label name
          [:input {:on-change (fn [e] (on-change (.-value (.-target e))))
                   :type :range
                   :value value
                   :min 0 :max 255}]
          [:input {:on-change (fn [e] (on-change (.-value (.-target e))))
                   :value value}]]

    :else nil))


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
     [:pre (with-out-str (pprint schema))]
     [:pre (with-out-str (pprint value))]
     (*render schema
              (fn [x] (p x))
              value)

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

      ;;initial render
      (full-render!))))


(defn main
  []
  (start! $app-container {}))
