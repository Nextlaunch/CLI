pub async fn launch_json() {
    let mut sequence: i64 = 0;

    print(format!("{}, \"sequence\":{}{}", "{\"opcode\":0, \"data\":{\"message\":\"Nextlaunch is starting\"}", sequence, '}'));
    sequence+=1;

    let client = reqwest::Client::new();
    let mut last = Instant::now();

    let mut log: Vec<(DateTime<Local>, String, u8)> = vec![];
    print(format!("{}, \"sequence\":{}{}", "{\"opcode\":0, \"data\":{\"message\":\"Nextlaunch is starting\"}", sequence, '}'));
    sequence+=1;


    let mut launch: Option<Launch> = update(&client, &mut log).await;
    let mut news: Option<Vec<Article>> = news_update(&client, &mut log).await;

    if launch.is_some() && news.is_some() {
        log.push((Local::now(), "updated launch and news caches".to_string(), 0));
    } else if launch.is_some() && news.is_none() {
        log.push((Local::now(), "updated launch cache".to_string(), 0));
    } else if launch.is_none() && news.is_some() {
        log.push((Local::now(), "updated news cache".to_string(), 0));
    }

    let mut needs_refresh = false;
    let mut refresh_cycle: i32 = 0;

    loop {
        refresh_cycle += 1;

        if needs_refresh {
            let temp_launch = update(&client, &mut log).await;
            let temp_news = news_update(&client, &mut log).await;
            if temp_launch.is_some() {
                launch = temp_launch;
            }
            if temp_news.is_some() {
                news = temp_news;
            }

            if launch.is_some() && news.is_some() {
                log.push((Local::now(), "updated launch and news caches".to_string(), 0));
            } else if launch.is_some() && news.is_none() {
                log.push((Local::now(), "updated launch cache".to_string(), 0));
            } else if launch.is_none() && news.is_some() {
                log.push((Local::now(), "updated news cache".to_string(), 0));
            }
            needs_refresh = false;
        }


        if refresh_cycle >= 4 {
            refresh_cycle = 0;
        }


        if last.elapsed().as_secs() > 60 * 10 {
            last = Instant::now();
            needs_refresh = true;
        }

        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}