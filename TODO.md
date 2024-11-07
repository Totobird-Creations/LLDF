### DiamondFire 'primitive' types:
- [x] String
- [x] Text
- [ ] Number
- [ ] Location
- [ ] Vector
- [ ] Sound
- [ ] Particle
- [ ] Potion
- [ ] Item
- [ ] List
- [ ] Dict

### LLDF-rs 'higher-level' types:
- [ ] UInt
- [ ] Int
- [ ] Float
- [ ] List
- [ ] Dict
- [ ] PlayerSel
- [ ] EntitySel

### LLVM types:
- [ ] Struct
- [ ] Arrays
- [ ] Enum
- [ ] Union
- [ ] Function
  - [x] The actual function
  - [ ] Function pointers
- [ ] Pointers

### The things that will be pain to implement:
- [x] Stack allocation
- [ ] Heap allocation
- [x] Pointers to allocated space
- [ ] Pointers to elements in other values (lists, dicts)
- [ ] Dynamic action codeblock tags
- [ ] Flow control
  - [ ] Loops
  - [ ] Conditionals
  - [x] Calling
- [ ] Panicking
- [ ] Signed vs unsigned integers

### Codegen:
- [x] Basic template generation
- [ ] Template optimisation
  - [x] Dead selections
  - [x] Duplicate selections
  - [x] Redundant equals
  - [x] Substitutable arithmetic
  - [x] Substitutable string
  - [x] Substitutable text
- [ ] Template splitting
- [ ] Specify CCAPI behaviour
  - [ ] Clear plot
  - [ ] Swap templates