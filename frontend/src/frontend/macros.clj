(ns frontend.macros
  (:require fipp.edn))

(defmacro p
  "Print and return native JavaScript argument."
  [x]
  `(let [res# ~x]
     (~@(if (:ns &env) '[.log js/console] '[prn]) res#)
     res#))

(defmacro pp
  "Pretty print and return argument."
  [x]
  `(let [res# ~x]
     (~@(if (:ns &env) '[.log js/console] '[prn]) (with-out-str (fipp.edn/pprint res#)))
     res#))

(defmacro timeout
  [delay & body]
  `(js/setTimeout (fn [] ~@body) ~delay))

(defmacro interval
  [delay & body]
  `(js/setInterval (fn [] ~@body) ~delay))
