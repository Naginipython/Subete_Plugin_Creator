function getChapterPages(html) {
    html = JSON.parse(html); 
    let hash = html['chapter']['hash'];
    let data = html['chapter']['data'];
    return data.map(x => `https://uploads.mangadex.org/data/${hash}/${x}`);
}