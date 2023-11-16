// The final functionality will look like this:

// A blog post starts as an empty draft.
// When the draft is done, a review of the post is requested.
// When the post is approved, it gets published.
// Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
use blog_oop::Post;
fn main() {
    let mut post = Post::new();
}