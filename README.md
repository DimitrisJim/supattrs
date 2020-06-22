### Pending

Support for writting macro attributes.

#### Notes:

In order to generalize the application of attributes to items, two general
tools can be used (as a first thought on this):

- An item visiter, visits each item and its members recursively.
- An attribute applicator (is that even a word!?) that knows how to apply
  itself on a given item (i.e on a module, a function, a struct etc.)

The visitor should accept an applicator (or, in general, some object that
accepts the item representation and transforms it) and apply it recursively
as it visits the subitems of an item.

After this, we need an entry point, though. Would a single function that just calls
`visit` (or, whatever the name is) do? We could parse the input as an `Item` and
then call `match` but that would result in a big `match` block that calls visit
for every item; that's non-extensible and it seems like a cleaner solution
could be found. 

#### Licence

Licensed under either of:
 
 - Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
 - MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
 
at your option.