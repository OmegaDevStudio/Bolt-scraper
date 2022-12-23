#![allow(non_snake_case)]
use futures::{stream::futures_unordered::FuturesUnordered, StreamExt};
use reqwest::Client;

use serde_json::json;
use tokio::time::{sleep, Duration};
use serde::Deserialize;


const GRAPHQL: &str = "https://replit.com/graphql";

// For ids

#[derive(serde_query::Deserialize, Debug)]
struct Startid {
    #[query(".[0].data.repl.id")]
    id: String,
}


// For forks

#[derive(serde_query::Deserialize, Debug)]
struct StartFork {
    #[query(".[0].data.repl.publicForkCount")]
    publicForkCount: usize,
    #[query(".[0].data.repl.publicForks.items")]
    items: Vec<MainArray>,
}


#[derive(Deserialize, Debug)]
struct MainArray {
    url: String,
    id: String,
}



#[derive(Clone)]
struct ReplUrl {
    url: String,
}

impl ReplUrl {
    fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    async fn fetch_id(&self) -> String {
        let json = json!([
            {
                "operationName":"ReplView",
                "variables":{
                    "url":&self.url
                },
                "query":"query ReplView($url: String!) {\n  repl(url: $url) {\n    ... on Repl {\n      id\n      imageUrl\n      ...ReplViewRepl\n      __typename\n    }\n    __typename\n  }\n  currentUser {\n    id\n    ...ReplViewCurrentUser\n    __typename\n  }\n}\n\nfragment ReplViewRepl on Repl {\n  id\n  title\n  timeCreated\n  imageUrl\n  publicReleasesForkCount\n  publicForkCount\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  relatedRepls(limitPerGroup: 3) {\n    name\n    repls {\n      id\n      publishedAs\n      ...ReplLinkRepl\n      ...TemplateReplCardRepl\n      ...ReplPostReplCardRepl\n      __typename\n    }\n    __typename\n  }\n  lang {\n    id\n    displayName\n    __typename\n  }\n  currentUserPermissions {\n    containerWrite\n    publish\n    changeIconUrl\n    __typename\n  }\n  publishedAs\n  deployment {\n    id\n    activeRelease {\n      id\n      timeCreated\n      __typename\n    }\n    __typename\n  }\n  ...ReplViewReplTitleRepl\n  ...ReplViewReplViewerRepl\n  ...ReplLinkRepl\n  ...ReplViewFooterRepl\n  __typename\n}\n\nfragment ReplLinkRepl on Repl {\n  id\n  url\n  nextPagePathname\n  __typename\n}\n\nfragment TemplateReplCardRepl on Repl {\n  id\n  iconUrl\n  templateCategory\n  title\n  description(plainText: true)\n  publicReleasesForkCount\n  templateLabel\n  likeCount\n  url\n  owner {\n    ... on User {\n      id\n      ...TemplateReplCardFooterUser\n      __typename\n    }\n    ... on Team {\n      id\n      ...TemplateReplCardFooterTeam\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment TemplateReplCardFooterUser on User {\n  id\n  username\n  image\n  url\n  __typename\n}\n\nfragment TemplateReplCardFooterTeam on Team {\n  id\n  username\n  image\n  url\n  __typename\n}\n\nfragment ReplPostReplCardRepl on Repl {\n  id\n  iconUrl\n  description(plainText: true)\n  ...ReplPostReplInfoRepl\n  ...ReplStatsRepl\n  ...ReplLinkRepl\n  tags {\n    id\n    ...PostsFeedNavTag\n    __typename\n  }\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostReplInfoRepl on Repl {\n  id\n  title\n  description(plainText: true)\n  imageUrl\n  iconUrl\n  templateInfo {\n    label\n    iconUrl\n    __typename\n  }\n  __typename\n}\n\nfragment ReplStatsRepl on Repl {\n  id\n  likeCount\n  runCount\n  commentCount\n  __typename\n}\n\nfragment PostsFeedNavTag on Tag {\n  id\n  isOfficial\n  __typename\n}\n\nfragment ReplViewReplTitleRepl on Repl {\n  id\n  title\n  iconUrl\n  templateInfo {\n    iconUrl\n    __typename\n  }\n  owner {\n    ... on User {\n      id\n      username\n      __typename\n    }\n    ... on Team {\n      id\n      username\n      __typename\n    }\n    __typename\n  }\n  ...ReplViewReplActionsPermissions\n  __typename\n}\n\nfragment ReplViewReplActionsPermissions on Repl {\n  id\n  lastPublishedAt\n  publishedAs\n  templateReview {\n    id\n    promoted\n    __typename\n  }\n  currentUserPermissions {\n    publish\n    __typename\n  }\n  ...UnpublishReplRepl\n  __typename\n}\n\nfragment UnpublishReplRepl on Repl {\n  id\n  commentCount\n  likeCount\n  runCount\n  publishedAs\n  __typename\n}\n\nfragment ReplViewReplViewerRepl on Repl {\n  id\n  publishedAs\n  runCount\n  publicForkCount\n  publicReleasesForkCount\n  prodUrl: hostedUrl(dotty: true)\n  isProject\n  nextPagePathname\n  lang {\n    id\n    header\n    displayName\n    __typename\n  }\n  ...ReplViewerOutputOverlayRepl\n  ...UseReplViewerRepl\n  ...LikeButtonRepl\n  __typename\n}\n\nfragment ReplViewerOutputOverlayRepl on Repl {\n  id\n  title\n  imageUrl\n  lastPublishedAt\n  currentUserPermissions {\n    changeImageUrl\n    __typename\n  }\n  __typename\n}\n\nfragment UseReplViewerRepl on Repl {\n  id\n  previewUrl: hostedUrl(dotty: false, dev: false)\n  url\n  wasPosted\n  wasPublished\n  publishedAs\n  isProject\n  lang {\n    id\n    canUseShellRunner\n    hasReplboxWebview\n    __typename\n  }\n  config {\n    isServer\n    isVnc\n    __typename\n  }\n  deployment {\n    id\n    activeRelease {\n      id\n      previewUrl: hostedUrl\n      __typename\n    }\n    __typename\n  }\n  replViewSettings {\n    id\n    defaultView\n    replFile\n    __typename\n  }\n  ...CrosisContextRepl\n  __typename\n}\n\nfragment CrosisContextRepl on Repl {\n  id\n  language\n  slug\n  user {\n    id\n    username\n    __typename\n  }\n  currentUserPermissions {\n    containerWrite\n    __typename\n  }\n  flagOwnerDotReplitPackager: gateOnOwner(feature: \"flag-dotreplit-packager\")\n  __typename\n}\n\nfragment LikeButtonRepl on Repl {\n  id\n  currentUserDidLike\n  likeCount\n  url\n  wasPosted\n  wasPublished\n  __typename\n}\n\nfragment ReplViewFooterRepl on Repl {\n  id\n  description\n  lastPublishedAt\n  publishedAs\n  deployment {\n    id\n    activeRelease {\n      id\n      timeCreated\n      __typename\n    }\n    __typename\n  }\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      followerCount\n      isFollowedByCurrentUser\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      followerCount\n      isFollowedByCurrentUser\n      __typename\n    }\n    __typename\n  }\n  source {\n    release {\n      id\n      __typename\n    }\n    deployment {\n      id\n      repl {\n        id\n        ...ReplViewSourceRepl\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n  tags {\n    id\n    __typename\n  }\n  origin {\n    id\n    ...ReplViewSourceRepl\n    __typename\n  }\n  __typename\n}\n\nfragment ReplViewSourceRepl on Repl {\n  id\n  iconUrl\n  title\n  templateLabel\n  ...ReplLinkRepl\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment ReplViewCurrentUser on CurrentUser {\n  id\n  username\n  isSubscribed\n  isModerator: hasRole(role: MODERATOR)\n  isAdmin: hasRole(role: ADMIN)\n  ...ReplViewReplViewerCurrentUser\n  __typename\n}\n\nfragment ReplViewReplViewerCurrentUser on CurrentUser {\n  id\n  ...LikeButtonCurrentUser\n  ...CrosisContextCurrentUser\n  __typename\n}\n\nfragment LikeButtonCurrentUser on CurrentUser {\n  id\n  isVerified\n  __typename\n}\n\nfragment CrosisContextCurrentUser on CurrentUser {\n  id\n  username\n  isSubscribed\n  flagTrackOtClientDataLoss: gate(feature: \"flag-ot-data-loss-client-tracking\")\n  flagPid1Ping: gate(feature: \"flag-pid1-ping-sample\")\n  flagNoPongReconnect: gate(feature: \"flag-no-pong-reconnect\")\n  __typename\n}\n"
            }
        ]);
        let client = Client::new();
        let resp = client.post(GRAPHQL)
        .header("host", "replit.com")
        .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0")
        .header("origin", "https://replit.com")
        .header("connection", "keep-alive")
        .header("cookie", "_anon_id=abf0f575-1c1c-4de7-bc67-7803533ea9b5; connect.sid=s%3AMX1T-6foGoJBZ-vD4mai6zi3B5PP-__N.%2BikcOEv6Hxvjef1mdu3cBMV0SHttLuzNwL0Tf6rEDi8; replit:authed=1; replit_authed=1; gating_id=0c813224-211d-4b4b-a0c5-552f7d6e41c4; ajs_anonymous_id=0c813224-211d-4b4b-a0c5-552f7d6e41c4; gating_id=0c813224-211d-4b4b-a0c5-552f7d6e41c4; ld_uid=4425394; __stripe_mid=5e9068e7-c2cc-4cdf-92e2-2e6799b7973ef5d0a3; cf_clearance=vRhSia.1nw.phCZ.l3IyiXHIanb4D1vhUro0g38LMdg-1664479523-0-150; amplitudeSessionId=1667485526; sidebarClosed=true; _dd_s=logs=1&id=92c31f11-b9fb-42b9-8d23-7420b2ce2481&created=1667485526231&expire=1667486587104&rum=0")
        .header("x-requested-with", "XMLHttpRequest")
        .json(&json)
        .send().await;
        match resp {
            Ok(resp) => resp.json::<Startid>().await.unwrap().id,
            Err(e) => panic!("Could not get Repl ID: {e}"),
        }
    }

    async fn fetch_forks(&self, id: &str) -> Vec<String> {
        let json = json!([
            {
                "operationName":"ReplViewForks",
                "variables":{
                    "replId": id,
                    "count":500
                },
                "query":"query ReplViewForks($replId: String!, $count: Int!, $after: String) {\n  repl(id: $replId) {\n    ... on Repl {\n      id\n      publicForkCount\n      publicReleasesForkCount\n      publicForks(count: $count, after: $after) {\n        items {\n          id\n          ...ReplPostReplCardRepl\n          __typename\n        }\n        pageInfo {\n          nextCursor\n          __typename\n        }\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment ReplPostReplCardRepl on Repl {\n  id\n  iconUrl\n  description(plainText: true)\n  ...ReplPostReplInfoRepl\n  ...ReplStatsRepl\n  ...ReplLinkRepl\n  tags {\n    id\n    ...PostsFeedNavTag\n    __typename\n  }\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostReplInfoRepl on Repl {\n  id\n  title\n  description(plainText: true)\n  imageUrl\n  iconUrl\n  templateInfo {\n    label\n    iconUrl\n    __typename\n  }\n  __typename\n}\n\nfragment ReplStatsRepl on Repl {\n  id\n  likeCount\n  runCount\n  commentCount\n  __typename\n}\n\nfragment ReplLinkRepl on Repl {\n  id\n  url\n  nextPagePathname\n  __typename\n}\n\nfragment PostsFeedNavTag on Tag {\n  id\n  isOfficial\n  __typename\n}\n"
            }
        ]);
        let client = Client::new();
        let resp = client
            .post(GRAPHQL)
            .header("host", "replit.com")
            .header(
                "user-agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0",
            )
            .header("origin", "https://replit.com")
            .header("connection", "keep-alive")
            .header("x-requested-with", "XMLHttpRequest")
            .json(&json)
            .send()
            .await
            .unwrap();

        let repl = resp.json::<StartFork>().await.unwrap();
        let count = repl.publicForkCount;
        println!("\x1b[0;92mFound {count} forks...\x1b[0m");
        let forks = repl.items;
        let mut urls: Vec<String> = vec![];
        let mut ids: Vec<String> = vec![];
        for fork in forks {
            urls.push(fork.url);
            ids.push(fork.id);
        }
        if ids.last().is_some() {
            loop {
                let json2 = json!([
                    {
                    "operationName":"ReplViewForks",
                        "variables":{
                            "replId": id,
                            "count":500,
                            "after": ids.last().unwrap()
                        },
                    "query":"query ReplViewForks($replId: String!, $count: Int!, $after: String) {\n  repl(id: $replId) {\n    ... on Repl {\n      id\n      publicForkCount\n      publicReleasesForkCount\n      publicForks(count: $count, after: $after) {\n        items {\n          id\n          ...ReplPostReplCardRepl\n          __typename\n        }\n        pageInfo {\n          nextCursor\n          __typename\n        }\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment ReplPostReplCardRepl on Repl {\n  id\n  iconUrl\n  description(plainText: true)\n  ...ReplPostReplInfoRepl\n  ...ReplStatsRepl\n  ...ReplLinkRepl\n  tags {\n    id\n    ...PostsFeedNavTag\n    __typename\n  }\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostReplInfoRepl on Repl {\n  id\n  title\n  description(plainText: true)\n  imageUrl\n  iconUrl\n  templateInfo {\n    label\n    iconUrl\n    __typename\n  }\n  __typename\n}\n\nfragment ReplStatsRepl on Repl {\n  id\n  likeCount\n  runCount\n  commentCount\n  __typename\n}\n\nfragment ReplLinkRepl on Repl {\n  id\n  url\n  nextPagePathname\n  __typename\n}\n\nfragment PostsFeedNavTag on Tag {\n  id\n  isOfficial\n  __typename\n}\n"
                    }
                ]);
                let resp = client
                    .post(GRAPHQL)
                    .header("host", "replit.com")
                    .header(
                        "user-agent",
                        "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0",
                    )
                    .header("origin", "https://replit.com")
                    .header("connection", "keep-alive")
                    .header("x-requested-with", "XMLHttpRequest")
                    .json(&json2)
                    .send()
                    .await
                    .unwrap();

                let repl = resp.json::<StartFork>().await;

                match repl {
                    Ok(repl) => {
                        let forks = repl.items;

                        if !forks.is_empty() {
                            for fork in forks {
                                urls.push(fork.url);
                                ids.push(fork.id);
                            }
                            println!("\x1b[0;34m{} forks loaded...\x1b[0m", urls.len());
                        } else {
                            break;
                        }
                    }
                    Err(_) => {
                        println!("Probably ratelimited, sleeping for 10 seconds");
                        sleep(Duration::from_secs(10)).await;
                    }
                }
            }
        }
        urls
    }
}

// For Global Scrape

#[derive(serde_query::Deserialize, Debug)]
struct StartGlobal {
    #[query(".[0].data.replPosts.items")]
    items: Vec<OtherArray>,
    #[query(".[0].data.replPosts.pageInfo.nextCursor")]
    nextCursor: Option<String>,
}

#[derive(Deserialize, Debug)]
struct OtherArray {
    repl: Repl3,
}

#[derive(Deserialize, Debug)]
struct Repl3 {
    url: String,
}



pub struct ReplGlobal {}

impl ReplGlobal {
    async fn fetch_urls(&self, query: &str, order: &str, max: u32) -> Vec<String> {

        let json = json!([
            {
                "operationName":"ReplPostsFeed",
                "variables":{
                    "options":{
                        "tags":[query],"order":order
                    }
                },
                "query":"query ReplPostsFeed($options: ReplPostsQueryOptions) {\n  currentUser {\n    id\n    ...ReplPostCurrentUser\n    __typename\n  }\n  replPosts(options: $options) {\n    pageInfo {\n      nextCursor\n      __typename\n    }\n    items {\n      id\n      ...ReplPostPost\n      ...ReplCardPostPost\n      ...OldPostPost\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment ReplPostCurrentUser on CurrentUser {\n  id\n  isModerator: hasRole(role: MODERATOR)\n  isAdmin: hasRole(role: ADMIN)\n  ...LikeButtonCurrentUser\n  __typename\n}\n\nfragment LikeButtonCurrentUser on CurrentUser {\n  id\n  isVerified\n  __typename\n}\n\nfragment ReplPostPost on Post {\n  id\n  title\n  timeCreated\n  isPinned\n  isAnnouncement\n  ...ReplViewPostActionPermissions\n  replComment {\n    id\n    body(removeMarkdown: true)\n    __typename\n  }\n  repl {\n    id\n    ...ReplViewReplActionsPermissions\n    ...ReplPostRepl\n    __typename\n  }\n  user {\n    id\n    ...ReplPostUserPostUser\n    __typename\n  }\n  recentReplComments {\n    id\n    ...ReplPostReplComment\n    __typename\n  }\n  __typename\n}\n\nfragment ReplViewPostActionPermissions on Post {\n  id\n  isHidden\n  __typename\n}\n\nfragment ReplViewReplActionsPermissions on Repl {\n  id\n  slug\n  lastPublishedAt\n  publishedAs\n  owner {\n    ... on User {\n      id\n      username\n      __typename\n    }\n    ... on Team {\n      id\n      username\n      __typename\n    }\n    __typename\n  }\n  templateReview {\n    id\n    promoted\n    __typename\n  }\n  currentUserPermissions {\n    publish\n    __typename\n  }\n  ...UnpublishReplRepl\n  __typename\n}\n\nfragment UnpublishReplRepl on Repl {\n  id\n  commentCount\n  likeCount\n  runCount\n  publishedAs\n  __typename\n}\n\nfragment ReplPostRepl on Repl {\n  id\n  ...ReplPostReplInfoRepl\n  ...LikeButtonRepl\n  ...ReplStatsRepl\n  tags {\n    id\n    ...PostsFeedNavTag\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostReplInfoRepl on Repl {\n  id\n  title\n  description(plainText: true)\n  imageUrl\n  iconUrl\n  templateInfo {\n    label\n    iconUrl\n    __typename\n  }\n  __typename\n}\n\nfragment LikeButtonRepl on Repl {\n  id\n  currentUserDidLike\n  likeCount\n  url\n  wasPosted\n  wasPublished\n  __typename\n}\n\nfragment ReplStatsRepl on Repl {\n  id\n  likeCount\n  runCount\n  commentCount\n  __typename\n}\n\nfragment PostsFeedNavTag on Tag {\n  id\n  isOfficial\n  __typename\n}\n\nfragment ReplPostUserPostUser on User {\n  id\n  username\n  image\n  ...UserLinkUser\n  __typename\n}\n\nfragment UserLinkUser on User {\n  id\n  url\n  username\n  __typename\n}\n\nfragment ReplPostReplComment on ReplComment {\n  id\n  body\n  timeCreated\n  user {\n    id\n    ...ReplPostRecentCommentUser\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostRecentCommentUser on User {\n  id\n  username\n  image\n  ...UserLinkUser\n  __typename\n}\n\nfragment ReplCardPostPost on Post {\n  id\n  title\n  timeCreated\n  isPinned\n  isAnnouncement\n  ...ReplViewPostActionPermissions\n  repl {\n    id\n    ...ReplViewReplActionsPermissions\n    ...ReplCardPostRepl\n    __typename\n  }\n  recentReplComments {\n    id\n    ...ReplPostReplComment\n    __typename\n  }\n  user {\n    id\n    ...ReplPostUserPostUser\n    __typename\n  }\n  __typename\n}\n\nfragment ReplCardPostRepl on Repl {\n  id\n  ...LikeButtonRepl\n  ...ReplPostReplCardRepl\n  recentComments {\n    id\n    ...ReplPostReplComment\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostReplCardRepl on Repl {\n  id\n  iconUrl\n  description(plainText: true)\n  ...ReplPostReplInfoRepl\n  ...ReplStatsRepl\n  ...ReplLinkRepl\n  tags {\n    id\n    ...PostsFeedNavTag\n    __typename\n  }\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment ReplLinkRepl on Repl {\n  id\n  url\n  nextPagePathname\n  __typename\n}\n\nfragment OldPostPost on Post {\n  id\n  title\n  preview(removeMarkdown: true, length: 150)\n  url\n  commentCount\n  isPinned\n  isAnnouncement\n  timeCreated\n  ...PostLinkPost\n  user {\n    id\n    ...ReplPostUserPostUser\n    __typename\n  }\n  repl {\n    id\n    ...ReplPostRepl\n    __typename\n  }\n  board {\n    id\n    name\n    color\n    __typename\n  }\n  recentComments(count: 3) {\n    id\n    preview(removeMarkdown: true, length: 500)\n    timeCreated\n    user {\n      id\n      ...ReplPostRecentCommentUser\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment PostLinkPost on Post {\n  id\n  url\n  __typename\n}\n"}

        ]);
        let client = Client::new();
        let resp = client
            .post(GRAPHQL)
            .header("host", "replit.com")
            .header(
                "user-agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0",
            )
            .header("origin", "https://replit.com")
            .header("connection", "keep-alive")
            .header("x-requested-with", "XMLHttpRequest")
            .json(&json)
            .send()
            .await
            .unwrap();

        let repl = resp.json::<StartGlobal>().await;

        let mut urls = vec![];
        let mut after = vec![];
        match repl {
            Ok(repl) => {
                after.push(repl.nextCursor);
                let data = repl.items;

                for items in data {
                    urls.push(items.repl.url);
                }
            }
            Err(e) => {
                println!("Probably ratelimited, sleeping for 10 seconds\n{e}");
                sleep(Duration::from_secs(10)).await;
            }
        }

        for _i in 0..max {
            if after.last().unwrap().is_some() {
                let json = json!([{
                    "operationName":"ReplPostsFeed",
                    "variables":{
                        "options":{
                            "tags":[query],
                            "order":order,
                            "after":after.last().unwrap()
                        }
                    },
                    "query":"query ReplPostsFeed($options: ReplPostsQueryOptions) {\n  currentUser {\n    id\n    ...ReplPostCurrentUser\n    __typename\n  }\n  replPosts(options: $options) {\n    pageInfo {\n      nextCursor\n      __typename\n    }\n    items {\n      id\n      ...ReplPostPost\n      ...ReplCardPostPost\n      ...OldPostPost\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment ReplPostCurrentUser on CurrentUser {\n  id\n  isModerator: hasRole(role: MODERATOR)\n  isAdmin: hasRole(role: ADMIN)\n  ...LikeButtonCurrentUser\n  __typename\n}\n\nfragment LikeButtonCurrentUser on CurrentUser {\n  id\n  isVerified\n  __typename\n}\n\nfragment ReplPostPost on Post {\n  id\n  title\n  timeCreated\n  isPinned\n  isAnnouncement\n  ...ReplViewPostActionPermissions\n  replComment {\n    id\n    body(removeMarkdown: true)\n    __typename\n  }\n  repl {\n    id\n    ...ReplViewReplActionsPermissions\n    ...ReplPostRepl\n    __typename\n  }\n  user {\n    id\n    ...ReplPostUserPostUser\n    __typename\n  }\n  recentReplComments {\n    id\n    ...ReplPostReplComment\n    __typename\n  }\n  __typename\n}\n\nfragment ReplViewPostActionPermissions on Post {\n  id\n  isHidden\n  __typename\n}\n\nfragment ReplViewReplActionsPermissions on Repl {\n  id\n  slug\n  lastPublishedAt\n  publishedAs\n  owner {\n    ... on User {\n      id\n      username\n      __typename\n    }\n    ... on Team {\n      id\n      username\n      __typename\n    }\n    __typename\n  }\n  templateReview {\n    id\n    promoted\n    __typename\n  }\n  currentUserPermissions {\n    publish\n    __typename\n  }\n  ...UnpublishReplRepl\n  __typename\n}\n\nfragment UnpublishReplRepl on Repl {\n  id\n  commentCount\n  likeCount\n  runCount\n  publishedAs\n  __typename\n}\n\nfragment ReplPostRepl on Repl {\n  id\n  ...ReplPostReplInfoRepl\n  ...LikeButtonRepl\n  ...ReplStatsRepl\n  tags {\n    id\n    ...PostsFeedNavTag\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostReplInfoRepl on Repl {\n  id\n  title\n  description(plainText: true)\n  imageUrl\n  iconUrl\n  templateInfo {\n    label\n    iconUrl\n    __typename\n  }\n  __typename\n}\n\nfragment LikeButtonRepl on Repl {\n  id\n  currentUserDidLike\n  likeCount\n  url\n  wasPosted\n  wasPublished\n  __typename\n}\n\nfragment ReplStatsRepl on Repl {\n  id\n  likeCount\n  runCount\n  commentCount\n  __typename\n}\n\nfragment PostsFeedNavTag on Tag {\n  id\n  isOfficial\n  __typename\n}\n\nfragment ReplPostUserPostUser on User {\n  id\n  username\n  image\n  ...UserLinkUser\n  __typename\n}\n\nfragment UserLinkUser on User {\n  id\n  url\n  username\n  __typename\n}\n\nfragment ReplPostReplComment on ReplComment {\n  id\n  body\n  timeCreated\n  user {\n    id\n    ...ReplPostRecentCommentUser\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostRecentCommentUser on User {\n  id\n  username\n  image\n  ...UserLinkUser\n  __typename\n}\n\nfragment ReplCardPostPost on Post {\n  id\n  title\n  timeCreated\n  isPinned\n  isAnnouncement\n  ...ReplViewPostActionPermissions\n  repl {\n    id\n    ...ReplViewReplActionsPermissions\n    ...ReplCardPostRepl\n    __typename\n  }\n  recentReplComments {\n    id\n    ...ReplPostReplComment\n    __typename\n  }\n  user {\n    id\n    ...ReplPostUserPostUser\n    __typename\n  }\n  __typename\n}\n\nfragment ReplCardPostRepl on Repl {\n  id\n  ...LikeButtonRepl\n  ...ReplPostReplCardRepl\n  recentComments {\n    id\n    ...ReplPostReplComment\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostReplCardRepl on Repl {\n  id\n  iconUrl\n  description(plainText: true)\n  ...ReplPostReplInfoRepl\n  ...ReplStatsRepl\n  ...ReplLinkRepl\n  tags {\n    id\n    ...PostsFeedNavTag\n    __typename\n  }\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment ReplLinkRepl on Repl {\n  id\n  url\n  nextPagePathname\n  __typename\n}\n\nfragment OldPostPost on Post {\n  id\n  title\n  preview(removeMarkdown: true, length: 150)\n  url\n  commentCount\n  isPinned\n  isAnnouncement\n  timeCreated\n  ...PostLinkPost\n  user {\n    id\n    ...ReplPostUserPostUser\n    __typename\n  }\n  repl {\n    id\n    ...ReplPostRepl\n    __typename\n  }\n  board {\n    id\n    name\n    color\n    __typename\n  }\n  recentComments(count: 3) {\n    id\n    preview(removeMarkdown: true, length: 500)\n    timeCreated\n    user {\n      id\n      ...ReplPostRecentCommentUser\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment PostLinkPost on Post {\n  id\n  url\n  __typename\n}\n"}]);

                let resp = client
                    .post(GRAPHQL)
                    .header("host", "replit.com")
                    .header(
                        "user-agent",
                        "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0",
                    )
                    .header("origin", "https://replit.com")
                    .header("connection", "keep-alive")
                    .header("x-requested-with", "XMLHttpRequest")
                    .json(&json)
                    .send()
                    .await
                    .unwrap();

                let repl = resp.json::<StartGlobal>().await;

                match repl {
                    Ok(repl) => {
                        after.push(repl.nextCursor);
                        let data = repl.items;

                        if !data.is_empty() {
                            for items in data {
                                urls.push(items.repl.url);
                            }
                        } else {
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Probably ratelimited, sleeping for 10 seconds\n{e}");
                        sleep(Duration::from_secs(10)).await;
                    },
                }

            }
        }
        urls
    }

    pub async fn fech_urls(&self, query: &str, max: u32) -> Vec<String> {
        let mut urls =vec![];

        urls.append(&mut self.fetch_urls(query, "Top",max).await);
        urls.append(&mut self.fetch_urls(query, "Hot",max).await);
        urls.append(&mut self.fetch_urls(query, "New",max).await);
        urls
    }
}

#[derive(serde_query::Deserialize, Debug)]
struct StartUser {
    #[query(".[0].data.user.profileRepls.pageInfo.nextCursor")]
    nextCursor: Option<String>,
    #[query(".[0].data.user.profileRepls.items")]
    items: Vec<MainArray>,
}



pub struct ReplUser {
    username: String,
}

impl ReplUser {
    pub fn new(username: &str) -> Self {
        Self { username : username.to_string() }
    }

    pub async fn fetch_urls(&self) -> Vec<String> {
        let json = json!([
            {
                "operationName":"ProfilePublicRepls",
                "variables":{
                    "username": &self.username,
                    "search":""
                },
                "query":"query ProfilePublicRepls($username: String!, $after: String, $search: String) {\n  user: userByUsername(username: $username) {\n    id\n    profileRepls: profileRepls(after: $after, search: $search) {\n      items {\n        id\n        ...ProfilePublicReplsRepl\n        __typename\n      }\n      pageInfo {\n        hasNextPage\n        nextCursor\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment ProfilePublicReplsRepl on Repl {\n  id\n  description(plainText: true)\n  isOwner\n  pinnedToProfile\n  timeCreated\n  title\n  url\n  iconUrl\n  ...ReplLinkRepl\n  user {\n    id\n    ...UserLinkUser\n    __typename\n  }\n  templateInfo {\n    label\n    iconUrl\n    __typename\n  }\n  multiplayers {\n    id\n    image\n    username\n    __typename\n  }\n  __typename\n}\n\nfragment ReplLinkRepl on Repl {\n  id\n  url\n  nextPagePathname\n  __typename\n}\n\nfragment UserLinkUser on User {\n  id\n  url\n  username\n  __typename\n}\n"
            }
        ]);
        let mut urls = vec![];
        let mut after = vec![];
        let client = Client::new();
        let resp = client
            .post(GRAPHQL)
            .header("host", "replit.com")
            .header(
                "user-agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0",
            )
            .header("origin", "https://replit.com")
            .header("connection", "keep-alive")
            .header("x-requested-with", "XMLHttpRequest")
            .json(&json)
            .send()
            .await;
        if let Ok(resp) = resp {
            let json = resp.json::<StartUser>().await;
            match json {
                Ok(json) => {
                    after.push(json.nextCursor);
                    for item in json.items {
                        urls.push(item.url);

                    }
                },
                Err(e) => {
                    println!("Probably ratelimited, sleeping for 10 seconds\n{e}");
                    sleep(Duration::from_secs(10)).await;
                },
            }
        }
        println!("\x1b[0;34m{} URLs loaded...\x1b[0m", urls.len());
        if after.last().unwrap().is_some() {
            loop {
                let json2 = json!([
                    {
                        "operationName":"ProfilePublicRepls",
                        "variables":{
                            "username": &self.username,
                            "search":"",
                            "after": after.last().unwrap(),
                        },
                        "query":"query ProfilePublicRepls($username: String!, $after: String, $search: String) {\n  user: userByUsername(username: $username) {\n    id\n    profileRepls: profileRepls(after: $after, search: $search) {\n      items {\n        id\n        ...ProfilePublicReplsRepl\n        __typename\n      }\n      pageInfo {\n        hasNextPage\n        nextCursor\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment ProfilePublicReplsRepl on Repl {\n  id\n  description(plainText: true)\n  isOwner\n  pinnedToProfile\n  timeCreated\n  title\n  url\n  iconUrl\n  ...ReplLinkRepl\n  user {\n    id\n    ...UserLinkUser\n    __typename\n  }\n  templateInfo {\n    label\n    iconUrl\n    __typename\n  }\n  multiplayers {\n    id\n    image\n    username\n    __typename\n  }\n  __typename\n}\n\nfragment ReplLinkRepl on Repl {\n  id\n  url\n  nextPagePathname\n  __typename\n}\n\nfragment UserLinkUser on User {\n  id\n  url\n  username\n  __typename\n}\n"
                    }
                ]);

                let resp = client
                    .post(GRAPHQL)
                    .header("host", "replit.com")
                    .header(
                        "user-agent",
                        "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0",
                    )
                    .header("origin", "https://replit.com")
                    .header("connection", "keep-alive")
                    .header("x-requested-with", "XMLHttpRequest")
                    .json(&json2)
                    .send()
                    .await;
                if let Ok(resp) = resp {
                    let json = resp.json::<StartUser>().await;
                    match json {
                        Ok(json) => {
                            let repl = json;
                            if repl.items.is_empty() {
                                break;
                            } else {
                                for item in repl.items {
                                    urls.push(item.url);
                                }
                                println!("\x1b[0;34m{} URLs loaded...\x1b[0m", urls.len());
                            }

                            if repl.nextCursor.is_none() {
                                break;
                            } else {
                                after.push(repl.nextCursor);
                            }
                        },
                        Err(e) => {
                            println!("Probably ratelimited, sleeping for 10 seconds\n{e}");
                            sleep(Duration::from_secs(10)).await;
                        },
                    }
                }

            }
        }
        urls
    }
}



pub struct ReplAPI {}

impl ReplAPI {
    pub async fn fetch_zips_url(&self, url: &str, max_count: u32) -> Vec<Vec<u8>> {
        let mut futs = FuturesUnordered::new();
        let repl = ReplUrl::new(url);
        let id = repl.fetch_id().await;
        let urls = repl.fetch_forks(&id).await;
        let mut urls = urls.into_iter().peekable();
        let client = Client::new();
        let mut count = 1;
        let mut chunk_count = 0;
        let mut zips = vec![];
        while let Some(url) = urls.next() {
            futs.push(fetch_zip(client.clone(), url, count));
            count += 1;
            chunk_count += 1;
            if urls.peek().is_none()
                || chunk_count >= 50
                || chunk_count >= max_count
                || count >= max_count
            {
                while let Some(val) = futs.next().await {
                    if let Some(val) = val {
                        zips.push(val)
                    }
                }
                chunk_count = 0;
            }
            if count > max_count {
                break;
            }
        }
        println!("\x1b[0;32mZips Loaded: {}", zips.len());
        zips
    }

    pub async fn fetch_urls_global(&self, urls: Vec<String>) -> Vec<String> {
        let mut all_urls = vec![];
        for url in urls {
            let repl = ReplUrl::new(&url);
            let id = repl.fetch_id().await;
            sleep(Duration::from_secs(3)).await;
            all_urls.append(&mut repl.fetch_forks(&id).await);
        }
        all_urls
    }

    pub async fn fetch_zips_user(&self, username: &str) -> Vec<Vec<u8>> {
        let repl = ReplUser::new(username);
        let urls = repl.fetch_urls().await;
        let mut count = 1;
        let mut chunk_count = 0;
        let mut zips = vec![];
        let client = Client::new();
        let mut urls = urls.into_iter().peekable();
        let mut futs = FuturesUnordered::new();
        while let Some(url) = urls.next() {
            futs.push(fetch_zip(client.clone(), url, count));
            count += 1;
            chunk_count += 1;
            if urls.peek().is_none()
                || chunk_count >= 50
            {
                while let Some(val) = futs.next().await {
                    if let Some(val) = val {
                        zips.push(val)
                    }
                }
                chunk_count = 0;
            }
        }
        zips
    }

    pub async fn fetch_zips_with_forks_user(&self, username: &str, max_count: u32) -> Vec<Vec<u8>> {
        let repl = ReplUser::new(username);
        let urls = repl.fetch_urls().await;
        let mut count = 1;
        let mut chunk_count = 0;
        let mut zips = vec![];
        let client = Client::new();
        let mut urls = urls.into_iter().peekable();
        let mut other_urls = urls.clone();
        let mut futs = FuturesUnordered::new();
        while let Some(url) = urls.next() {
            futs.push(fetch_zip(client.clone(), url.clone(), count));
            count += 1;
            chunk_count += 1;
            if urls.peek().is_none()
                || chunk_count >= 50
            {
                while let Some(val) = futs.next().await {
                    if let Some(val) = val {
                        zips.push(val)
                    }
                }
                chunk_count = 0;
            }
        }
        while let Some(url) = other_urls.next() {
            let repl = ReplUrl::new(&url);
            let id = repl.fetch_id().await;
            let urls = repl.fetch_forks(&id).await;
            let mut fork_urls = urls.into_iter().peekable();
            let mut count = 1;
            let mut chunk_count = 0;
            while let Some(url) = fork_urls.next() {
                futs.push(fetch_zip(client.clone(), url, count));
                count += 1;
                chunk_count += 1;
                if fork_urls.peek().is_none()
                    || chunk_count >= 50
                    || chunk_count >= max_count
                    || count >= max_count
                {
                    while let Some(val) = futs.next().await {
                        if let Some(val) = val {
                            zips.push(val)
                        }
                    }
                    chunk_count = 0;
                }
                if count > max_count {
                    break;
                }
            }
        }

        zips
    }


}

pub async fn fetch_zip(client: Client, url: String, count: u32) -> Option<Vec<u8>> {
    let url = format!("https://replit.com{url}.zip");
    println!("\x1b[0;93mStarted downloading URL {}...\x1b[0m", &count);
    loop {
        let resp = client.get(&url)
        .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("accept-encoding", "gzip, deflate, br")
        .header("accept-language", "en-GB,en-US;q=0.9,en;q=0.8")
        .header("sec-ch-ua", r#""Chromium";v="102", "Not A;Brand";v="99""#)
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", r#""Linux""#)
        .header("sec-fetch-dest", "document")
        .header("sec-fetch-mode", "navigate")
        .header("sec-fetch-site", "none")
        .header("sec-fetch-user", "?1")
        .header("service-worker-navigation-preload", "true")
        .header("upgrade-insecure-requests", "1")
        .header("cookie", "__stripe_mid=6b01b6ed-256d-495a-aca8-dd68f98fc9ca21169a; _ga=GA1.2.1243856305.1648378772; hubspotutk=b9dfe9ccc62bcc6659a535eda8a6ca1e; _anon_id=7300c6b8-7bae-4e1e-a15b-2a4047274bd3; connect.sid=s%3A6LRDoSdkMScBUxzuZoBPqjEt6kckQuvw.g2K3xWy1mGzdUwBi669kmf%2F05XQEVN0WS%2F7qAqVm9Rk; __hstc=205638156.b9dfe9ccc62bcc6659a535eda8a6ca1e.1648379787477.1659889475635.1662832129098.4; ajs_user_id=9914868; ajs_anonymous_id=ef4fefba-2f0a-4b12-9a20-e9067b0d5552; replit_ng=1663167550.58.41.725703|8035451343a2d8f3e54599c71b2aec19; replit:authed=1; replit_authed=1; _gid=GA1.2.406234258.1663167554; amplitudeSessionId=1663167554; sidebarClosed=true; _gat=1; __stripe_sid=cd50a6e5-b2d1-496c-85a5-e9d225ef87d963cd4d; _dd_s=logs=1&id=2e1a650d-d3d8-459b-87ee-b695ce1d32d9&created=1663167554666&expire=1663168491454&rum=0")
        .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.115 Safari/537.36")
        .send().await;
        match resp {
            Ok(resp) => {
                if resp.status().is_success() {
                    let mut src = vec![];
                    let mut bytes = resp.bytes_stream();
                    while let Some(bytes) = bytes.next().await {
                        match bytes {
                            Ok(bytes) => {
                                src.append(&mut bytes.to_vec());
                            },
                            Err(e) => {
                                println!("\x1b[0;91mError: {e}\x1b[0m");
                                return None;
                            }
                        }

                    }
                    println!("\x1b[0;34mFinished downloading URL {}...\x1b[0m", &count);
                    return Some(src);

                } else if resp.status().as_str() == "429" {
                    match resp.headers().get("retry-after") {
                        Some(retry) => {
                            println!(
                                "\x1b[0;91mRatelimited... Waiting for {} seconds\x1b[0m",
                                retry.to_str().unwrap()
                            );
                            sleep(Duration::from_secs(
                                retry.to_str().unwrap().to_string().parse::<u64>().unwrap(),
                            ))
                            .await;
                        }
                        None => return None,
                    }
                } else {
                    println!(
                        "\x1b[0;91mFailed to retrieve zip. Error: {}\x1b[0m",
                        resp.status().as_str()
                    );
                    return None;
                }
            }
            Err(_) => continue,
        }
    }
}
