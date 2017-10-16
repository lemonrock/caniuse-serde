# caniuse-serde

[caniuse-serde] is a rust crate wrapping access to the [caniuse database]. It uses Serde 1.0 to parse the JSON. It does not create a build-time perfect hash map, so allowing alternative versions of the [caniuse database] to be supported.

It ships with an up-to-date [caniuse database] which is embedded by default; currently version `1.0.30000746`.


## Licensing

The license for this project is MIT. The [caniuse database] is available for use under a [CC BY 4.0 license]. Attribution for the caniuse database is "caniuse.com". Questions regarding the [caniuse database] can be asked at <http://a.deveria.com/contact>.


[caniuse-serde]: https://github.com/lemonrock/caniuse-serde "caniuse-serde GitHub page"
[caniuse database]: https://github.com/Fyrd/caniuse "caniuse database GitHub page"
[CC BY 4.0 license]: https://creativecommons.org/licenses/by/4.0/ "CC BY 4.0 license information"
