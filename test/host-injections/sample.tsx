// Same queries as JavaScript, exercised against the TSX grammar
const a = (
  <FormattedMessage
    id="welcome"
    defaultMessage="Welcome to {site}, {name}!"
    values={{ site: "My App", name }}
  />
);
const b = intl.formatMessage({ defaultMessage: "{n, number} items" });
const n1 = <p title="just a title">text</p>;
