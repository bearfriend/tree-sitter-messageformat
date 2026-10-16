// Injected: shaped like an ICU argument
const a = "You have {count, plural, one {# item} other {# items}}";
const b = `Hello {name, select, a {A} other {B}}`;

// Injected: well-known constructors and functions
const c = new IntlMessageFormat("Plain {n} text");
const d = intl.formatMessage("Hi {name}!");
const e = { defaultMessage: "Welcome, {user}!" };

// Injected: explicit hint comment
const f = /* icu */ "Bye {who}";

// Not injected
const n1 = "just a string";
const n2 = `template ${x} {literal}`;
const n3 = "{not: 'icu'}";
const n4 = format("Hello {name}");
const n5 = { message: "Plain {name}" };

// JSX (needs injections/jsx.scm)
const g = <FormattedMessage defaultMessage="Welcome to {site}!" />;
const h = <p title="just a title">{"{n, number} items"}</p>;
