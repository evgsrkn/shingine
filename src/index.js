async function search(query) {
    const results = document.getElementById("files")
    results.innerHTML = "";
    const response = await fetch("/search", {
        method: 'POST',
        headers: {'Content-Type': 'text/plain'},
        body: query,
    });
    const json = await response.json();
    results.innerHTML = "";
    for (const key in json) {
        let item = document.createElement("a");
      item.setAttribute("href", "~" + key);
        item.appendChild(document.createTextNode(key));
        item.appendChild(document.createElement("br"));
        results.appendChild(item);
    }
}

let query = document.getElementById("query");
let currentSearch = Promise.resolve()

query.addEventListener("keypress", (e) => {
    if (e.key == "Enter") {
        currentSearch.then(() => search(query.value));
    }
})
