use googletest::matcher::Matcher;
use std::{fmt::Debug, marker::PhantomData};
use tonic::{Extensions, Request, Response};

pub fn extension<T: StoresExtensions + Debug, E: Debug>(
    inner: impl Matcher<Option<E>>,
) -> impl Matcher<T> {
    ExtensionMatcher {
        inner,
        phantom_t: Default::default(),
    }
}

struct ExtensionMatcher<T, InnerMatcherT> {
    inner: InnerMatcherT,
    phantom_t: PhantomData<T>,
}

impl<T: Debug, InnerMatcherT: Matcher<T>> Matcher<Option<E>>
    for ExtensionMatcher<T, InnerMatcherT>
{
    fn matches(&self, actual: &Response<T>) -> MatcherResult {
        self.inner.matches(actual.get_ref())
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Matches => {
                format!(
                    "has message proto which {}",
                    self.inner.describe(MatcherResult::Matches)
                )
            }
            MatcherResult::DoesNotMatch => {
                format!(
                    "has message proto which {}",
                    self.inner.describe(MatcherResult::DoesNotMatch)
                )
            }
        }
    }
}

trait StoresExtensions {
    fn get_extensions(&self) -> &Extensions;
}

impl<T> StoresExtensions for Request<T> {
    fn get_extensions(&self) -> &Extensions {
        self.extensions()
    }
}

impl<T> StoresExtensions for Response<T> {
    fn get_extensions(&self) -> &Extensions {
        self.extensions()
    }
}
