use std::fmt::Debug;
use std::marker::PhantomData;

use googletest::matcher::{Matcher, MatcherResult};
use tonic::Request;

pub fn request_message<T: Debug>(inner: impl Matcher<T>) -> impl Matcher<Request<T>> {
    RequestMessageMatcher {
        inner,
        phantom_t: Default::default(),
    }
}

struct RequestMessageMatcher<T, InnerMatcherT> {
    inner: InnerMatcherT,
    phantom_t: PhantomData<T>,
}

impl<T: Debug, InnerMatcherT: Matcher<T>> Matcher<Request<T>>
    for RequestMessageMatcher<T, InnerMatcherT>
{
    fn matches(&self, actual: &Request<T>) -> MatcherResult {
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
