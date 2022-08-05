# TODO: RunPack-Obj Tutorial

## 7. Objects

There is still one data type we haven't covered yet: the object. In RunPack an object is a set of key-value pairs, internally implemented with a hash map. Key and value can be of any type, integer, float, string, boolean, word, block, even another object.

To define an object, we use the `new` word:

```
( 'name' 'Joe'
  'phone' 5555555 new )
print_stack
```

Output:

```
Stack:
	0 : Object(Object { map: {String("phone"): Integer(5555555), String("name"): String("Joe")} })
```

As always, we can store it in a word using `def`:

```
( 'name' 'Andreu'
  'phone' 5555555 new ) def my_obj
```

We use a pair of words to `set` and `get` values from an object:

```
( 'name' 'Andreu'
  'phone' 5555555 new ) def my_obj

'name' @ my_obj get print
'name' 'Joe' @ my_obj set
'name' @ my_obj get print
```

Output:

```
Andreu
Joe
```

These operators use a [word reference](#6-word-references) to acces the object, to avoid cloning it in the stack over and over again.

We can check if a key exists in an object with the `key?` word:

```
( 'name' 'Andreu'
  'phone' 5555555 new ) def my_obj

'name' @ my_obj key? print
'xxxx' @ my_obj key? print
```

Output:

```
true
false
```

Vectors are just normal objects, with the particularity of having integer keys. They are defined with the `vec` word, but operated with the same words we use for any object:

```
( 12.34, 'A string', 1000, true vec ) def my_vec
1 @ my_vec get print
@ my_vec len print
```

Output:

```
A string
4
```

There is also an operator to "run" keys, the `:` word:

```
(
    'name' 'Andreu'
    'hi' { 'Hello, World!' print }
    new
) def my_obj

@ my_obj : 'hi'
@ my_obj : 'name' print
```

Output:

```
Hello, World!
Andreu
```

And an operator to run keys as if they were methods, passing a reference to the object in the stack:

```
{ dup : val_a } def get_a
{ swap : val_b } def get_b
(
    @ +         { get_a get_b + }
    @ val_a     10
    @ val_b     20
    new
)
def my_obj

@ my_obj . + print
```

Output:

```
30
```

In this case we are using words as keys instead of strings. The key `+` contains a block. When executed using "`.`" it uses the object reference in the stack to obtains the values of `val_a` and `val_b`, and add them.