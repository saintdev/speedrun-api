use serde::Serializer;

pub(crate) fn serialize_as_csv<S, T>(
    iter: impl IntoIterator<Item = T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: Into<&'static str>,
    S: Serializer,
{
    let out: Vec<_> = iter.into_iter().map(Into::into).collect();
    serializer.serialize_str(&out.join(","))
}
