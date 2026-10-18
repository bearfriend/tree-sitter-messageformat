// A comment
// <- comment
greeting: "Hello {name}!"
// <- property
//        ^ string
//         ^ string
//               ^ punctuation.bracket
//                ^ variable
items: `{count, plural, offset:1 =0 {none} one {# item} other {# items}}`
// <- property
//       ^ variable
//              ^ keyword
//                      ^ keyword
//                             ^ number
//                               ^ operator
//                                   ^ string
//                                         ^ constant.builtin
//                                              ^ variable.special
//                                                      ^ constant.builtin
sel: "{g, select, male {He} other {They}}"
//        ^ keyword
//                ^ constant
//                          ^ constant.builtin
"quoted.key": "{d, date, short}"
//     ^ property
//                 ^ type
//                       ^ constant.builtin
n: "{v, number, ::currency/USD}"
//      ^ type
//              ^ operator
//                ^ string.special
t: "<b>{x}</b>"
//   ^ tag
e: "it''s '{x}'"
//    ^ string.escape
//        ^ string.escape
