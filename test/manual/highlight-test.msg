// Test file for MessageFormat highlighting
// This demonstrates various syntax elements

simple: "Hello {name}!",

number_format: "Price: {amount, number, ::currency/USD}",

date_format: "Today is {date, date, ::MMM d, y}",

plural_basic: "{count, plural, =0 {no items} =1 {one item} other {# items}}",

plural_offset: "{guests, plural, offset:1 =0 {no guests} =1 {one guest} other {# guests}}",

select_gender: "{person, select, male {He} female {She} other {They}} went home",

selectordinal: "You finished {place, selectordinal, =1 {1st} =2 {2nd} =3 {3rd} other {#th}}",

template_string: `Welcome {name}!
You have {count, plural, =0 {no messages} other {# messages}}.`,

escaped_chars: "Use '{' and '}' for literal braces",

nested_complex: "{user, select, admin {Admin {name} has {tasks, plural, =0 {no tasks} other {# tasks}}} other {Regular user}}",

tags: "Click <button>here</button> or <link>this link</link>",

"quoted-key": "This key is quoted",

style_formats: "Short: {date, date, short}, Long: {date, date, long}"