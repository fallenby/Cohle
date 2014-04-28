pub enum ThingType {
    Comment, 
    Account,
    Link,
    Message,
    Subreddit,
    Award,
    PromoCampaign
}

impl ThingType
{
    pub fn get(&self) -> &'static str
    {
        match *self
        {
            Comment => "t1",
            Account => "t2",
            Link => "t3",
            Message => "t4",
            Subreddit => "t5",
            Award => "t6",
            PromoCampaign => "t8"
        }
    }
}
