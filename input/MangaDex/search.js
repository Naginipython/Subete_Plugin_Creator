function search(html) {
    html = JSON.parse(html);
    let data = [];
        for (let d of html['data']) {
            let temp = {};
            temp['id'] = d['id'];
            if (d['attributes']['title'].hasOwnProperty('en')) {
                temp['title'] = d['attributes']['title']['en'];
            } else if (d['attributes']['title'].hasOwnProperty('ja-ro')) {
                temp['title'] = d['attributes']['title']['ja-ro'];
            } else {
                temp['title'] = d['attributes']['title']['ja'];
            }
            let filetemp = d['relationships'].filter(o => o.type == 'cover_art')[0];
            if (filetemp != undefined) {
                temp['img'] = `https://uploads.mangadex.org/covers/${temp['id']}/${filetemp['attributes']['fileName']}`;
            } else {
                temp['img'] = '';
            }
            temp['plugin'] = 'MangaDex';
            temp['description'] = d['attributes']['description']['en']? d['attributes']['description']['en'] : '';
            temp['chapters'] = [];
            let author_names = d['relationships'].filter(x => x.type == 'author').map(y => y['attributes']['name']);
            let artist_names = d['relationships'].filter(x => x.type == 'artist').map(y => y['attributes']['name']);
            temp['authors'] = author_names.join(', ');
            temp['artists'] = artist_names.join(', ');
            data.push(temp);
        }
    return data;
}