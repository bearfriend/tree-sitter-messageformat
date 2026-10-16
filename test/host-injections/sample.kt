fun f() {
    MessageFormat("Hello {0}")
    MessageFormat.format("Bye {0}", 1)
    val a = "{0, plural, one {# file} other {# files}}"
    val b = "plain {0}"
    val c = """{0, select, a {A} other {B}}"""
    String.format("Not {0}", 1)
}
