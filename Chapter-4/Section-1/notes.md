# Ownership Notes

## Ownership Rules

* Each value in Rust has a variable that’s called its *owner*.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

## Stack-Only Data

* If a type has the *Copy* trait, an older variable is still usable after assignment
* Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait
* As a general rule, any group of simple scalar values can be Copy

## Heap Data

* *drop* function deallocates heap memory
* drop function is called when leaving the scope
* When a heap allocated value is reassigned, the original variable is invalidated preventing *double free* error
* *clone* method creates value copy (heap allocation and assignment)
