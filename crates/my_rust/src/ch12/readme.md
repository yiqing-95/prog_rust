

The standard library defines Eq as an extension of PartialEq, adding no new methods:
trait Eq: PartialEq<Self> {}