### Types:
- `int`   : Number
- `float` : Number
- `ptr`   : [ String, Number ]     <- Pointer types are represented using two element lists.
                                      The first element is the name of an `unsaved` variable.
                                      The second is the 'pointer shift'. Since pointer shifting
                                      isn't something DiamondFire can handle, it is just
                                      suffixed onto the name of the `unsaved` variable.
- `func`  : String
