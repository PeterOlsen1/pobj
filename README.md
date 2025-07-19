# Pobj

Peter's objects, basically just a JSON object.

What I need to do:
* Create macros to easily call get/set similar to JSON objects
* resize function when load factor is like > 70%
* upsert method
* Figure out true generics

I'd like to make it support some things:
* use from multiple threads (this might not really be necessary, unless I want to make multithreaded support in the future)
* network access
* distributed? (this would be a challenge)
