// NOTE: NO COMMENTS. BREAKS CODE. DELETE THESE ONCE FINISHED
// Takes in HTML, without \n, whitespace, and discarded some ascii
// returns: [ String ] , String will be link to img
function getChapterPages(html) {
    let data = [];
    const regex = new RegExp(`[your regex here](.*?)[ending regex]`, 'gi');
    let match;
    while ((match = regex.exec(html)) !== null) {
        data.push(match[1]);
    }
    return data;
}