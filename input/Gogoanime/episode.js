function getEpisodes(json, html) {
    json = JSON.parse(json);
    const jsonRegex = new RegExp(`<div class="anime_info_body_bg">(.*?)</p> </div>`, 'gi');
    const infoArea = jsonRegex.exec(html)[1];
    const descRegex = new RegExp(`"description"><p>(.*?)</p>`);
    json.description = descRegex.exec(infoArea)[1];
    const statusRegex = new RegExp(`Status: </span>(.*?)>(.*?)</a>`);
    json.status = statusRegex.exec(infoArea)[2];

    const idRegex = new RegExp(`<input type="hidden" value="(.*?)" id="movie_id"`);
    let id = idRegex.exec(html)[1];
    const endEpRegex = new RegExp(`ep_end = '(.*?)'`);
    let endEp = endEpRegex.exec(html)[1];
    json.extra = {
        next: `https://ajax.gogocdn.net/ajax/load-list-episode?ep_start=0&ep_end=${endEp}&id=${id}`
    };
    return json;
}

function next(json, html) {
    json = JSON.parse(json);
    delete json.extra;
    let data = [];
    const epRegex = new RegExp(`<li>(.*?)</li>`, 'gi');
    let match;
    while ((match = epRegex.exec(html)) != null) {
        let ep = {};
        const idRegex = new RegExp(`<a href=" /(.*?)"`);
        ep.id = idRegex.exec(match)[1];
        const numRegex = new RegExp(`</span> (.*?)</div>`);
        ep.number = parseInt(numRegex.exec(match)[1]);
        ep.title = "";
        ep.watch_time = 0;
        ep.completed = false;
        data.push(ep);
    }
    json.episodes = data;
    return json;
}