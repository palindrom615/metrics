use metrics::Unit;

pub(crate) trait UnitExt {
    fn as_ucum_label(&self) -> &'static str;
}

impl UnitExt for Unit {
    /// Gets the notation of Unified Code for Units of Measure (UCUM) for given unit
    ///
    /// This is useful for metric systems using UCUM, like [OpenTelemetry](https://opentelemetry.io/docs/specs/semconv/general/metrics/#instrument-units)
    ///
    /// See Also <https://ucum.org/>
    fn as_ucum_label(&self) -> &'static str {
        match self {
            // dimensionless
            Unit::Count            => "1",
            Unit::Percent          => "%",

            // time
            Unit::Seconds          => "s",
            Unit::Milliseconds     => "ms",
            Unit::Microseconds     => "us",
            Unit::Nanoseconds      => "ns",

            // storage (binary prefixes)
            Unit::Tebibytes        => "TiBy",
            Unit::Gibibytes        => "GiBy",
            Unit::Mebibytes        => "MiBy",
            Unit::Kibibytes        => "KiBy",
            Unit::Bytes            => "By",

            // network throughput
            Unit::TerabitsPerSecond => "Tbit/s",
            Unit::GigabitsPerSecond => "Gbit/s",
            Unit::MegabitsPerSecond => "Mbit/s",
            Unit::KilobitsPerSecond => "kbit/s",
            Unit::BitsPerSecond     => "bit/s",

            // event rate
            Unit::CountPerSecond    => "1/s",
        }
    }
}