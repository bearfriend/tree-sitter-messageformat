class Sample {
  void f() {
    new MessageFormat("Hello {0}");
    MessageFormat.format("Bye {0}", 1);
    String a = "{0, plural, one {# file} other {# files}}";
    String b = "plain {0}";
    String c = """
        {0, select, a {A} other {B}}
        """;
    String.format("Not {0}", 1);
  }
}
