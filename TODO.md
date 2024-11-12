### DiamondFire 'primitive' types:
- [x] String
- [x] Text
- [x] Number
- [x] Location
- [x] Vector
- [x] Sound
- [x] Particle
- [x] Potion
- [x] Item
- [x] List
- [ ] Dict

### LLDF-rs 'higher-level' types:
- [ ] UInt
- [ ] Int
- [ ] Float
- [x] UUID
- [x] Colour
- [x] PlayerSel
- [ ] EntitySel
- [ ] Instant
- [ ] Duration

### LLVM types:
- [ ] Struct
- [ ] Arrays
- [ ] Enum
- [x] Union
- [ ] Function
  - [x] Direct function
  - [ ] Indirect pointers
- [x] Pointers


### Fun types cause I feel like it:
- [x] Matrix
- [ ] Quat

### The things that will be pain to implement:
- [x] Stack allocation
- [ ] Heap allocation
- [x] Pointers to allocated space
- [ ] Pointers to elements in other values (lists, dicts)
- [x] Dynamic action codeblock tags
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
