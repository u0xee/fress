;; Copyright (c) Cole Frederick. All rights reserved.
;; The use and distribution terms for this software are covered by the
;; Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
;; which can be found in the file epl-v10.html at the root of this distribution.
;; By using this software in any fashion, you are agreeing to be bound by the terms of this license.
;; You must not remove this notice, or any other, from this software.

(ns fress.specs
  (:require [clojure.spec.alpha :as s]
            [clojure.spec.gen.alpha :as gen]
            [clojure.spec.test.alpha :as stest]
            [clojure.pprint :refer [pprint]]
            [clojure.java.shell :refer [sh]])
  (:import (java.lang ProcessBuilder Process)))

(def {})
(println "Hello there you")
(def x 7)

(gen/sample (s/gen int?))
(gen/sample (s/gen (s/every int? :into #{})))
(pprint (gen/sample (s/gen (s/every int? :into #{}))))
(s/def :code/blue (s/every int? :into #{}))
(s/explain :code/blue #{1, 2, 3.})

(defn say-hello []
      (println "Say-hello there!"))
(defn -main []
  (println "Hello from -main"))

;(stest/check `plus-one {:clojure.spec.test.check/opts {:num-tests 5000}})

;; Idea
; Rust binary with echo endpoint
; Drive fn stub that defers to Rust binary
; Drive fn checks

"
Spec
Specize
abbrev
alt
alt-impl
amp-impl
and
and-spec-impl
assert
assert*
cat
cat-impl
check-asserts
check-asserts?
coll-of
conform
conform*
conformer
def
def-impl
describe
describe*
double-in
every
every-impl
every-kv
exercise
exercise-fn
explain
explain*
explain-data
explain-data*
explain-out
explain-printer
explain-str
fdef
form
fspec
fspec-impl
gen
gen*
get-spec
inst-in
inst-in-range?
int-in
int-in-range?
invalid?
keys
keys*
map-of
map-spec-impl
maybe-impl
merge
merge-spec-impl
multi-spec
multi-spec-impl
nilable
nilable-impl
nonconforming
or
or-spec-impl
regex-spec-impl
regex?
registry
rep+impl
rep-impl
spec
spec-impl
spec?
specize*
tuple
tuple-impl
unform
unform*
valid?
with-gen
with-gen*


(pprint (gen/sample (gen/large-integer) 40))
any
any-printable
bind
boolean
bytes
cat
char
char-alpha
char-alphanumeric
char-ascii
choose
delay
delay-impl
double
double*
elements
fmap
for-all*
frequency
gen-for-name
gen-for-pred
generate
hash-map
int
keyword
keyword-ns
large-integer
large-integer*
lazy-combinator
lazy-combinators
lazy-prim
lazy-prims
list
map
not-empty
one-of
quick-check
ratio
return
sample
set
shuffle
simple-type
simple-type-printable
string
string-alphanumeric
string-ascii
such-that
symbol
symbol-ns
tuple
uuid
vector
vector-distinct"

