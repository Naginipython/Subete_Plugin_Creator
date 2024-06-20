function getChapterPages(html) {
    let data = [];
    const regex = new RegExp(`<img src='(.*?)'`, 'gi');
    let match;
    while ((match = regex.exec(html)) !== null) {
        data.push(match[1]);
    }
    return data;
}