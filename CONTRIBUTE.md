# Contribute info
## Naming convention

| Item                        | Convention                                            |
|-----------------------------|-------------------------------------------------------|
| Crates                      | unclear                                               |
| Modules                     | snake_case                                            |
| Local variables             | snake_case                                            |
| Statics, Constants          | SCREAMING_SNAKE_CASE                                  |
| Types, Enum, Traits         | UpperCamelCase                                        |
| Functions, Methods          | snake_case                                            |
| Macros                      | snake_case!                                           |
| General constructors        | new or with_more_details                              |
| Conversion constructors     | from_some_other_type                                  |
| Type parameters             | concise UpperCamelCase, usually single letter: T      |
| Lifetimes                   | short lowercase, usually a single letter: 'a          |
| Features                    | unclear but see C-FEATURE                             |