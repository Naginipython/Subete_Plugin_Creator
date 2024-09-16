function getEpisodeVideo(html) {
    let dataVideo = html.match(new RegExp('data-video="(.*?)"'))[1];
    return { next: dataVideo };
}

function next(json, html) {
    json = JSON.parse(json);
    iv = html.match(new RegExp('wrapper container-(.*?)"'))[1];
    secretKey = html.match(new RegExp('body class="container-(.*?)"'))[1];
    decryptionKey = html.match(new RegExp('videocontent-(.*?)"'))[1];
    let dataValue = html.match(new RegExp('data-value="(.*?)"'))[1];

    const idRegex = new RegExp(`data-video="https://(.*?)id=(.*?)&`);
    const id = idRegex.exec(html)[2];

    let host = "https://" + "s3taku.com";
    return { 
        next: "BUILD",
        decrypt: {
            string: dataValue,
            iv: iv,
            key: secretKey,
        },
        encrypt: {
            string: id,
            iv: iv,
            key: secretKey
        },
        decryptionKey: decryptionKey,
        build: `${host}/encrypt-ajax.php?id=$encrypt&$decrypt&alias=${id}`
    };
}

function next2(json, html) {
    json = JSON.parse(json);
    let data = JSON.parse(html).data;
    return {
        next: "CRYPTO",
        decrypt: {
            string: data,
            iv: json.decrypt.iv,
            key: json.decryptionKey
        }
    }
}

function next3(json, html) {
    json = JSON.parse(json);
    let sourceList = JSON.parse(json.data);
    return sourceList.source[0].file;
}