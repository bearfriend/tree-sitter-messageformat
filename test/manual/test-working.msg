// Simple placeholder
greeting: "Hello {name}!",

// Number formatting
price: "Cost: {amount, number, ::currency/USD}",

// Date with skeleton pattern
birthday: "Born on {date, date, ::MMM d, y}",

// Time with skeleton
meeting: "Meeting at {time, time, ::h:mm a}",

// Basic plural
items: "{count, plural, =0 {no items} =1 {one item} other {# items}}",

// Plural with offset
guests: "{count, plural, offset:1 =0 {no guests} =1 {one guest} other {# guests}}",

// Select format
gender: "{person, select, male {He} female {She} other {They}} is here",

// Nested placeholders
complex: "{count, plural, =0 {No {type} available} =1 {One {type} left} other {# {type} items}}",

// Selectordinal
position: "You finished {place, selectordinal, =1 {1st} =2 {2nd} =3 {3rd} other {#th}}",

// Template string with multiline
welcome: `Welcome {name}! 
You have {msgCount, plural, =0 {no messages} =1 {one message} other {# messages}}.`,

// Escaped characters
escaped: "Use '{' and '}' for literal braces, '#' for pound",

// Multiple formats
status: "{user, select, admin {Admin {name} has {count, plural, =0 {no tasks} other {# tasks}}} user {User {name}} other {Guest}}"