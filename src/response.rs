use std::fmt::Debug;
use std::marker::PhantomData;

use googletest::matcher::{Matcher, MatcherResult};
use tonic::Response;

pub fn response_message<T: Debug>(inner: impl Matcher<T>) -> impl Matcher<Response<T>> {
    ResponseMessageMatcher {
        inner,
        phantom_t: Default::default(),
    }
}

struct ResponseMessageMatcher<T, InnerMatcherT> {
    inner: InnerMatcherT,
    phantom_t: PhantomData<T>,
}

impl<T: Debug, InnerMatcherT: Matcher<T>> Matcher<Response<T>>
    for ResponseMessageMatcher<T, InnerMatcherT>
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
