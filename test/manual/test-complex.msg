// Simple placeholder
simple: "Hello {name}!",

// Plural with offset
guests: "{count, plural, offset:1 =0 {no guests} =1 {one guest} other {# guests}}",

// Date with skeleton
birthday: "Your birthday is {date, date, ::MMM d, y}",

// Time with skeleton
meeting: "Meeting at {time, time, ::h:mm a}",

// Select format
gender: "{person, select, male {He} female {She} other {They}} went home",

// Nested messages
complex: "{count, plural, =0 {No items} =1 {One {type} item} other {# {type} items available}}",

// Multiple nesting levels
nested: "{user, select, admin {Welcome {name}, you have {count, plural, =0 {no messages} =1 {one message} other {# messages}}} guest {Hello {name}} other {Hi there}}",

// Number formatting
price: "Price: {amount, number, currency}",

// Selectordinal
position: "You finished {place, selectordinal, =1 {1st} =2 {2nd} =3 {3rd} other {#th}}",

// Complex skeleton patterns
datetime: "Event: {timestamp, date, ::EEEE, MMMM d, y 'at' h:mm a}",

// Escaped characters
escaped: "Use \"backslash\" escapes for conflicting quotes",

// Mixed content
mixed: `Welcome {name}! You have {msgCount, plural,
  =0 {no new messages}
  =1 {one new message from {sender}}
  other {# new messages, latest from {sender}}
}. Your next {eventType, select,
  meeting {meeting is at {eventTime, time, ::h:mm a}}
  call {call is scheduled for {eventTime, date, ::MMM d}}
  other {event is {eventTime, date, ::EEEE}}
}`,

// Offset with pound placeholder
countdown: "{remaining, plural, offset:1 =0 {Time's up!} =1 {Only # second left} other {# seconds remaining}}"

// Simple message ending with a single quote
simple: "Hello {name}'",
