#!/bin/bash

# shellcheck disable=SC2016

curl 'https://api.fontawesome.com/' -X POST -H 'Content-Type: application/json' --data-raw '{"query":" \nquery {release(version: \"6.4.0\") {\nicons {\nid\nunicode\nfamilyStylesByLicense {\nfree {\nfamily\nstyle\n}\n}\n}\n}\n}\n","variables":null}' \
| jq --compact-output '[.data.release.icons[] | select(.familyStylesByLicense.free | length > 0) | { style: .familyStylesByLicense.free[].style, id: .id, unicode: .unicode }]' \
| node -e 'console.log((() => {
    function id(i) { 
        let ident = i.split("-").map(x=>x.toUpperCase()).join("_"); 
        if(ident[0].match(/\d/)) return "ICON_" + ident;
        else return ident;
    }

    function isUnique(data, id) {
        return data.filter(x=>x.id == id).length == 1;
    } 

    let data = JSON.parse(fs.readFileSync(0).toString());

    return data.map(x=>
        `pub static ${id(x.id)}${isUnique(data, x.id) ? "" : "_" + x.style.toUpperCase()}: crate::Icon = ( "\\u{${x.unicode}}", crate::FONT_${x.style.toUpperCase()} );`
    ).join("\n");
})())'