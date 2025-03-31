(self.webpackChunk_N_E = self.webpackChunk_N_E || []).push([[7926], {
    44383: e => {
        var i = {
            kind: "Document",
            definitions: [{
                kind: "OperationDefinition",
                operation: "query",
                name: {
                    kind: "Name",
                    value: "GET_SOCIAL_SHOPPINGS"
                },
                variableDefinitions: [{
                    kind: "VariableDefinition",
                    variable: {
                        kind: "Variable",
                        name: {
                            kind: "Name",
                            value: "take"
                        }
                    },
                    type: {
                        kind: "NamedType",
                        name: {
                            kind: "Name",
                            value: "Int"
                        }
                    },
                    directives: []
                }, {
                    kind: "VariableDefinition",
                    variable: {
                        kind: "Variable",
                        name: {
                            kind: "Name",
                            value: "latest"
                        }
                    },
                    type: {
                        kind: "NamedType",
                        name: {
                            kind: "Name",
                            value: "String"
                        }
                    },
                    directives: []
                }],
                directives: [],
                selectionSet: {
                    kind: "SelectionSet",
                    selections: [{
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "socialShopping"
                        },
                        arguments: [{
                            kind: "Argument",
                            name: {
                                kind: "Name",
                                value: "take"
                            },
                            value: {
                                kind: "Variable",
                                name: {
                                    kind: "Name",
                                    value: "take"
                                }
                            }
                        }, {
                            kind: "Argument",
                            name: {
                                kind: "Name",
                                value: "latest"
                            },
                            value: {
                                kind: "Variable",
                                name: {
                                    kind: "Name",
                                    value: "latest"
                                }
                            }
                        }],
                        directives: [],
                        selectionSet: {
                            kind: "SelectionSet",
                            selections: [{
                                kind: "Field",
                                name: {
                                    kind: "Name",
                                    value: "latestTransactionTimeStamp"
                                },
                                arguments: [],
                                directives: []
                            }, {
                                kind: "Field",
                                name: {
                                    kind: "Name",
                                    value: "items"
                                },
                                arguments: [],
                                directives: [],
                                selectionSet: {
                                    kind: "SelectionSet",
                                    selections: [{
                                        kind: "FragmentSpread",
                                        name: {
                                            kind: "Name",
                                            value: "SocialShoppingItem"
                                        },
                                        directives: []
                                    }]
                                }
                            }]
                        }
                    }]
                }
            }, {
                kind: "FragmentDefinition",
                name: {
                    kind: "Name",
                    value: "SocialShoppingItem"
                },
                typeCondition: {
                    kind: "NamedType",
                    name: {
                        kind: "Name",
                        value: "SocialShoppingItem"
                    }
                },
                directives: [],
                selectionSet: {
                    kind: "SelectionSet",
                    selections: [{
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "id"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "userName"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "cityName"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "dateTime"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "imageUrl"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "brandName"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "fullProductName"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "displayPrice"
                        },
                        arguments: [],
                        directives: [],
                        selectionSet: {
                            kind: "SelectionSet",
                            selections: [{
                                kind: "Field",
                                name: {
                                    kind: "Name",
                                    value: "amountInclusive"
                                },
                                arguments: [],
                                directives: []
                            }, {
                                kind: "Field",
                                name: {
                                    kind: "Name",
                                    value: "amountExclusive"
                                },
                                arguments: [],
                                directives: []
                            }, {
                                kind: "Field",
                                name: {
                                    kind: "Name",
                                    value: "currency"
                                },
                                arguments: [],
                                directives: []
                            }]
                        }
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "oAuthProviderName"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "targetUserName"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "quote"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "voteTypeId"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "productTypeName"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "socialShoppingTransactionTypeId"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "url"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "rating"
                        },
                        arguments: [],
                        directives: []
                    }, {
                        kind: "Field",
                        name: {
                            kind: "Name",
                            value: "searchString"
                        },
                        arguments: [],
                        directives: []
                    }]
                }
            }],
            loc: {
                start: 0,
                end: 538
            }
        };
        i.loc.source = {
            body: "query GET_SOCIAL_SHOPPINGS($take: Int, $latest: String) {\n  socialShopping(take: $take, latest: $latest) {\n    latestTransactionTimeStamp\n    items {\n      ...SocialShoppingItem\n    }\n  }\n}\n\nfragment SocialShoppingItem on SocialShoppingItem {\n  id\n  userName\n  cityName\n  dateTime\n  imageUrl\n  brandName\n  fullProductName\n  displayPrice {\n    amountInclusive\n    amountExclusive\n    currency\n  }\n  oAuthProviderName\n  targetUserName\n  quote\n  voteTypeId\n  productTypeName\n  socialShoppingTransactionTypeId\n  url\n  rating\n  searchString\n}\n",
            name: "GraphQL request",
            locationOffset: {
                line: 1,
                column: 1
            }
        };
        var n = {};
        function t(e, i) {
            for (var n = 0; n < e.definitions.length; n++) {
                var t = e.definitions[n];
                if (t.name && t.name.value == i)
                    return t
            }
        }
        function a(e, i) {
            var a = {
                kind: e.kind,
                definitions: [t(e, i)]
            };
            e.hasOwnProperty("loc") && (a.loc = e.loc);
            var r = n[i] || new Set
              , l = new Set
              , o = new Set;
            for (r.forEach(function(e) {
                o.add(e)
            }); o.size > 0; ) {
                var s = o;
                o = new Set,
                s.forEach(function(e) {
                    l.has(e) || (l.add(e),
                    (n[e] || new Set).forEach(function(e) {
                        o.add(e)
                    }))
                })
            }
            return l.forEach(function(i) {
                var n = t(e, i);
                n && a.definitions.push(n)
            }),
            a
        }
        i.definitions.forEach(function(e) {
            if (e.name) {
                var i = new Set;
                (function e(i, n) {
                    if ("FragmentSpread" === i.kind)
                        n.add(i.name.value);
                    else if ("VariableDefinition" === i.kind) {
                        var t = i.type;
                        "NamedType" === t.kind && n.add(t.name.value)
                    }
                    i.selectionSet && i.selectionSet.selections.forEach(function(i) {
                        e(i, n)
                    }),
                    i.variableDefinitions && i.variableDefinitions.forEach(function(i) {
                        e(i, n)
                    }),
                    i.definitions && i.definitions.forEach(function(i) {
                        e(i, n)
                    })
                }
                )(e, i),
                n[e.name.value] = i
            }
        }),
        e.exports = i,
        e.exports.GET_SOCIAL_SHOPPINGS = a(i, "GET_SOCIAL_SHOPPINGS"),
        e.exports.SocialShoppingItem = a(i, "SocialShoppingItem")
    }
    ,
    9672: (e, i, n) => {
        "use strict";
        n.d(i, {
            A: () => r
        });
        var t = n(31085)
          , a = n(53793);
        let r = e => {
            let {title: i, titleId: n, ...r} = e;
            return (0,
            t.jsxs)(a.E, {
                fill: "none",
                viewBox: "0 0 16 16",
                width: 16,
                height: 16,
                "aria-labelledby": n,
                ...r,
                children: [i ? (0,
                t.jsx)("title", {
                    id: n,
                    children: i
                }) : null, (0,
                t.jsx)("path", {
                    fill: "#000",
                    fillRule: "evenodd",
                    d: "M0 1v10.001h3V15h1l4-3.999h8V1zm1 1.001h14V10H7.586l-1 1.001L4 13.587V10H1z",
                    clipRule: "evenodd"
                })]
            })
        }
    }
    ,
    97468: (e, i, n) => {
        "use strict";
        n.d(i, {
            A: () => r
        });
        var t = n(31085)
          , a = n(53793);
        let r = e => {
            let {title: i, titleId: n, ...r} = e;
            return (0,
            t.jsxs)(a.E, {
                fill: "none",
                viewBox: "0 0 17 16",
                width: 17,
                height: 17,
                "aria-labelledby": n,
                ...r,
                children: [i ? (0,
                t.jsx)("title", {
                    id: n,
                    children: i
                }) : null, (0,
                t.jsx)("path", {
                    fill: "#000",
                    fillRule: "evenodd",
                    d: "M4 1v7h5l4 4.001h1.001V8h2V1zm1.001 1.001h10V7H13v3.586L9.414 7H5.001zM0 4.001v7h2V15h1l4.001-3.999H10V10H6.587L3 13.587V10H1V5.001h2v-1z",
                    clipRule: "evenodd"
                })]
            })
        }
    }
    ,
    70026: (e, i, n) => {
        "use strict";
        n.d(i, {
            A: () => r
        });
        var t = n(31085)
          , a = n(53793);
        let r = e => {
            let {title: i, titleId: n, ...r} = e;
            return (0,
            t.jsxs)(a.E, {
                fill: "none",
                viewBox: "0 0 16 16",
                width: 16,
                height: 16,
                "aria-labelledby": n,
                ...r,
                children: [i ? (0,
                t.jsx)("title", {
                    id: n,
                    children: i
                }) : null, (0,
                t.jsx)("path", {
                    fill: "#000",
                    fillRule: "evenodd",
                    d: "M13 15H3v-5c0-1.103.897-2 2-2h6c1.103 0 2 .897 2 2zm-2-8H5a3 3 0 0 0-3 3v6h12v-6a3 3 0 0 0-3-3M8 0a3 3 0 1 0 0 6 3 3 0 0 0 0-6m0 1c1.103 0 2 .897 2 2s-.897 2-2 2-2-.897-2-2 .897-2 2-2",
                    clipRule: "evenodd"
                })]
            })
        }
    }
    ,
    35664: (e, i, n) => {
        "use strict";
        n.d(i, {
            DN: () => o,
            Xp: () => s,
            gX: () => c,
            p9: () => d,
            r5: () => l
        });
        var t = n(39628)
          , a = n(74064)
          , r = n(85585);
        let l = a.Ay.div.attrs({
            "aria-hidden": !0
        }).withConfig({
            componentId: "sc-335b9445-0"
        })(["width:100%;", ';&::before{content:"\\200D";position:static;display:inline-block;width:0;}&::after{content:" ";position:static;display:inline-block;', ";width:100%;height:", "em;vertical-align:baseline;}"], t.C, r.J, e => {
            let {theme: i} = e;
            return "galaxus" === i.brand ? .5 : .54
        }
        );
        (0,
        a.Ay)(l).withConfig({
            componentId: "sc-335b9445-1"
        })(["", ";"], t.a_);
        let o = (0,
        a.Ay)(l).withConfig({
            componentId: "sc-335b9445-2"
        })(["", ";"], t.hS)
          , s = (0,
        a.Ay)(l).withConfig({
            componentId: "sc-335b9445-3"
        })(["", ";"], t.DD);
        (0,
        a.Ay)(l).withConfig({
            componentId: "sc-335b9445-4"
        })(["", ";"], t.Pm);
        let d = (0,
        a.Ay)(l).withConfig({
            componentId: "sc-335b9445-5"
        })(["", ";"], t.HI)
          , c = (0,
        a.Ay)(l).withConfig({
            componentId: "sc-335b9445-6"
        })(["", ";"], t.EX)
    }
    ,
    25681: (e, i, n) => {
        "use strict";
        n.r(i),
        n.d(i, {
            SocialShoppingListInternal: () => ii,
            default: () => io
        });
        var t = n(31085)
          , a = n(7293)
          , r = n(41085)
          , l = n(29309)
          , o = n(97867)
          , s = n(55969)
          , d = n(92800)
          , c = n(74064)
          , u = n(14041)
          , m = n(32889)
          , g = n(44383)
          , p = n(76462);
        let h = {}
          , v = (e, i) => {
            let n = new Map;
            return e.forEach(e => {
                n.set(`${e.userName}:${e.dateTime}:${e.socialShoppingTransactionTypeId}`, e)
            }
            ),
            i.forEach(e => {
                n.set(`${e.userName}:${e.dateTime}:${e.socialShoppingTransactionTypeId}`, {
                    ...e,
                    incoming: !0
                })
            }
            ),
            Array.from(n.values()).sort(N).slice(0, 7)
        }
          , N = (e, i) => {
            let n = new Date(i.dateTime).getTime() - new Date(e.dateTime).getTime();
            return 0 !== n ? n : e.incoming ? -1 : i.incoming ? 1 : 0
        }
        ;
        var f = n(76446)
          , S = n(99170)
          , T = n(74530)
          , y = n(10628)
          , b = function(e) {
            return e.SEARCH = "search",
            e.COMMUNITY = "community",
            e.PRODUCT = "product",
            e
        }({});
        let k = () => {
            let[e,i] = (0,
            u.useState)(!1)
              , n = (0,
            y.ar)()
              , {ref: t, inView: a} = (0,
            m.Wx)({
                threshold: .5
            });
            return (0,
            u.useEffect)( () => {
                a && !e && (i(!0),
                n({
                    elementImpressionObjectId: null,
                    elementContent: [Object.values(b)],
                    objectAction: "livefeed_impression",
                    elementType: null,
                    elementTarget: [Object.values(b)]
                }))
            }
            , [a]),
            t
        }
          , I = e => {
            let i = (0,
            y.ar)();
            return () => {
                i({
                    elementImpressionObjectId: null,
                    elementContent: [e],
                    objectAction: "livefeed_select",
                    elementType: null,
                    elementTarget: [e]
                })
            }
        }
          , A = {
            RATING: 1,
            USERCREATION: 2,
            ORDEREDPRODUCT: 3,
            PICKEDUPPRODUCT: 4,
            SHIPPEDPRODUCT: 5,
            WATCHEDPRODUCT: 6,
            SEARCH: 7,
            OAUTHUSERCONNECTED: 8,
            USERVOTESONRATING: 9,
            USERCOMMENTSONRATING: 10,
            USERASKSQUESTION: 11,
            USERANSWERSQUESTION: 12,
            USERVOTESONQUESTION: 13,
            USERVOTESONANSWER: 14,
            USERCOMMENTSONANSWER: 15,
            USERSTARTSDISCUSSION: 16,
            USERCONTRIBUTESTODISCUSSION: 17,
            USERVOTESONDISCUSSIONPOST: 18,
            USERLIKESBRAND: 19
        }
          , x = e => {
            let i = [A.ORDEREDPRODUCT, A.PICKEDUPPRODUCT, A.SHIPPEDPRODUCT, A.WATCHEDPRODUCT].indexOf(e) > -1
              , n = b.COMMUNITY;
            return i ? n = b.PRODUCT : e === A.SEARCH && (n = b.SEARCH),
            n
        }
          , P = ["[0] stellt eine [1] Frage zu [2]: [3]", "[0] is asking a [1] question about [2]: [3]", "[0] pose une question [1] sur l'article [2]: [3]", "[0] sta facendo una domanda [1] su [2]: [3]", "[0] stelt een [1] vraag over [2]: [3]"]
          , E = ["[0] findet die Antwort von [1] [2]: [3]", "[0] finds [1]'s answer [2]: [3]", "[0] trouve la r\xe9ponse de [1] [2]: [3]", "[0] trova la risposta di [1] [2]: [3]", "[0] vindt [1]'s antwoord [2]: [3]"]
          , C = ["[0] aus [1] sucht nach [2]", "[0] from [1] is looking for [2]", "[0] de [1] cherche [2]", "[0] di [1] sta cercando [2]", "[0] van [1] is op zoek naar [2]"]
          , w = ["[0] f\xfcr [1] wurde verschickt an [2] aus [3]", "[0] for [1] was sent to [2] from [3]", "[0] pour [1] a \xe9t\xe9 envoy\xe9(e) \xe0 [2] de [3]", "[0] per [1] \xe8 stato/a appena inviato/a a [2] da [3]", "[0] voor [1] is verzonden naar [2] van [3]"]
          , U = ["[0] [1] gef\xe4llt [2]", "[0] [1] likes [2]", "[0] [1] aime [2]", "A [0] [1] piace [2]", "[0] [1] houdt van [2]"]
          , R = ["[0] aus [1] bestellt [2] f\xfcr [3]", "[0] from [1] just ordered [2] for [3]", "[0] de [1] commande [2] pour [3]", "[0] di [1] ha appena ordinato [2] per [3]", "[0] van [1] net [2] besteld voor [3]"]
          , O = ["[0] aus [1] hat sein Konto mit [2] verbunden", "[0] from [1] connected his account with [2]", "[0] de [1] a connect\xe9 son compte avec [2]", "[0] di [1] ha collegato il suo account con [2]", "[0] van [1] verbond zijn rekening met [2]"]
          , j = ["[0] aus [1] schaut sich [2] f\xfcr [3] an", "[0] from [1] is looking at [2] for [3]", "[0] de [1] est en train de regarder [2] pour [3]", "[0] di [1] sta guardando [2] per [3]", "[0] van [1] kijkt naar [2] voor [3]"]
          , D = ["[0] kommentiert die [2] Antwort von [1]: [3]", "[0] commented on [1]'s [2] answer: [3]", "[0] commente la r\xe9ponse [2] de [1]: [3]", "[0] sta commentando la risposta [2] di [1]: [3].", "[0] reageerde op [1]'s [2] antwoord: [3]"]
          , F = ["[0] startet eine [1] Diskussion zu [2]: [3]", "[0] is starting a [1] discussion about [2]: [3]", "[0] commence une discussion [1] sur [2]: [3]", "[0] sta iniziando una discussione [1] su [2]: [3].", "[0] begint een [1] discussie over [2]: [3]"]
          , H = ["[0] aus [1] hat sich als [2] registriert", "[0] from [1] just registered as a [2]", "[0] de [1] s'est enregistr\xe9 en tant que [2]", "[0] di [1] si \xe8 appena registrato come [2]", "[0] van [1] net geregistreerd als een [2]"]
          , z = ["[0] [1] beantwortet die [3] Frage von [2]: [4]", "[0] [1] is answering [2]'s [3] question: [4]", "[0] [1] r\xe9pond \xe0 la question [3] de [2]: [4].", "[0] [1] sta rispondendo alla domanda [3] di [2]: [4]", "[0] [1] beantwoordt [2]'s [3] vraag: [4]"]
          , $ = ["[0] findet die Rezension von [1] [2]: [3]", "[0] found [1]'s review [2]: [3]", "[0] trouve l'avis de [1] [2]\xa0: [3]", "[0] trova la recensione di [1] [2]: [3]", "[0] vond [1]'s recensie [2]: [3]"]
          , L = ["[0] findet [1]'s [2] Beitrag [3]: [4]", "[0] finds [1]'s [2] post [3]: [4]", "[0] trouve le message [2] de [1] [3]: [4]", "[0] trova il messaggio [2] di [1] [3]: [4]", "[0] vindt [1]'s [2] post [3]: [4]"]
          , M = ["[0] beteiligt sich an einer Diskussion \xfcber [1]: [2]", "[0] is taking part in a discussion about [1]: [2]", "[0] prend part \xe0 une discussion sur [1]: [2]", "[0] sta prendendo parte alla discussione su [1]: [2].", "[0] neemt deel aan een discussie over [1]: [2]"]
          , W = ["nicht n\xfctzlich", "not useful", "pas utile", "non utile", "niet bruikbaar"]
          , q = ["neuer Kunde", "new customer", "nouveau client", "nuovo cliente", "nieuwe klant"]
          , G = ["n\xfctzlich", "useful", "utile", "utile", "nuttig"]
          , B = ["[0] aus [1] bewertet [2] mit [3]", "[0] from [1] rated [2] with [3]", "[0] de [1] \xe9value [2] avec [3]", "[0] di [1] ha appena valutato [2] con [3]", "[0] van [1] beoordeeld [2] met [3]"]
          , V = ["[0] findet die [2] Frage von [1] [3]: [4]", "[0] finds [1]'s [2] question [3]: [4]", "[0] trouve la question [2] de [1] [3]: [4]", "[0] trova la domanda [2] di [1] [3]: [4]", "[0] vindt [1]'s [2] vraag [3]: [4]"]
          , _ = ["missbr\xe4uchlich", "abusive", "abusif", "abusivo", "misbruik"]
          , Y = ["[0] [1] kommentiert die Bewertung von [2]: [3]", "[0] [1] commented on [2]'s rating: [3]", "[0] [1] commente l'\xe9valuation de [2]: [3]", "[0] [1] sta commentando la valutazione di [2] : [3]", "[0] [1] reageerde op [2]'s beoordeling: [3]"]
          , Z = ["[0] holt [1] f\xfcr [2] in unserer Filiale in [3] ab", "[0] is collecting [1] for [2] from our store in [3]", "[0] cherche [1] pour [2] dans notre succursale de [3]", "[0] ha appena scelto [1] per [2] nel nostro negozio a [3]", "[0] zamelt [1] in voor [2] van onze winkel in [3]"];
        var Q = n(70026)
          , K = n(24114)
          , X = n(9672)
          , J = n(53793);
        let ee = e => {
            let {title: i, titleId: n, ...a} = e;
            return (0,
            t.jsxs)(J.E, {
                fill: "none",
                viewBox: "0 0 16 16",
                width: 16,
                height: 16,
                "aria-labelledby": n,
                ...a,
                children: [i ? (0,
                t.jsx)("title", {
                    id: n,
                    children: i
                }) : null, (0,
                t.jsx)("path", {
                    fill: "#000",
                    fillRule: "evenodd",
                    d: "M16 4.062c0 .907-.492 1.34-1.185 1.928-.375.32-.395.341-.395.712V7h-1.272v-.535c0-.67.268-.815.962-1.392.395-.33.609-.527.609-1.01 0-.485-.46-.888-.993-.888-.92 0-1.261.991-1.261.991l-1.09-.486S11.889 2 13.726 2C15.008 2 16 2.916 16 4.061m-1.366 4.752a.826.826 0 0 1-.844.815c-.471 0-.845-.36-.845-.815 0-.453.374-.814.845-.814.47 0 .844.36.844.814M12 11h1V16H1v-6a3 3 0 0 1 3-3h6v1H4c-1.103 0-2 .897-2 2v5h10zM7 1C8.103 1 9 1.897 9 3a2 2 0 0 1-2 2c-1.103 0-2-.898-2-2 0-1.103.897-2 2-2m0 5A2.999 2.999 0 1 0 7 0a3 3 0 1 0 0 6",
                    clipRule: "evenodd"
                })]
            })
        }
        ;
        var ei = n(97468)
          , en = n(60766)
          , et = n(95363)
          , ea = n(15733)
          , er = n(27066)
          , el = n(24455)
          , eo = n(26139);
        let es = (0,
        c.Ay)(el._).withConfig({
            componentId: "sc-9116fb7d-0"
        })(["display:inline;", ";padding:", ";width:auto;", "{display:inline-block;vertical-align:middle;max-width:60px;max-height:28px;margin-right:", ';&[data-leading-image="false"]{margin-left:', ";}}"], eo.m, o.Y[4], l.s.desktopWidescreen, o.Y[8], o.Y[8])
          , ed = e => {
            let {src: i, alt: n, isLeadingImage: a=!1} = e;
            return (0,
            t.jsx)(es, {
                "data-leading-image": a ? "true" : "false",
                className: "social-shopping-image",
                src: i,
                alt: n || "",
                aspectRatio: "1/1",
                width: 50,
                optimizationPolicy: "product"
            })
        }
        ;
        var ec = n(62088)
          , eu = n(66977)
          , em = n(10666)
          , eg = n(39628);
        let ep = (0,
        c.AH)(["", "{", ";font-weight:300;}"], l.s.desktopWidescreen, eg.DD)
          , eh = (0,
        c.i7)(["0%{opacity:0;}100%{opacity:1;}"])
          , ev = c.Ay.div.withConfig({
            componentId: "sc-b8c78cd-0"
        })(["border-bottom:1px solid ", ";", ";opacity:0;position:absolute;transform:", ";transition:transform ", " ", ";animation:", " ", " forwards ", ";animation-delay:", ";width:100%;", " ", "{transition:none;animation-duration:0.01s;animation-delay:0s;}&:nth-child(4n){border-bottom:1px solid transparent;}&:nth-child(5n){a{display:none;}}"], ec.CR8, e => {
            let {$size: i} = e;
            return i ? (0,
            c.AH)(["line-height:1.8;"]) : (0,
            c.AH)(["line-height:1.6;"])
        }
        , e => {
            let {$position: i} = e;
            return `translateY(${100 * i}%)`
        }
        , eu.p0[500], eu.cz.standard, eh, eu.p0[500], eu.cz.standard, eu.cb[250], e => {
            let {$size: i} = e;
            return i === T.e.BIG && (0,
            c.AH)(["", ";"], ep)
        }
        , em.J)
          , eN = c.Ay.div.withConfig({
            componentId: "sc-b8c78cd-1"
        })(["display:flex;padding:", " 0;", " svg{position:relative;top:-2px;}span + img{margin-left:0;}a{inset:-1px ", " -1px ", ";border-bottom:1px solid ", ";border-top:1px solid ", ";width:auto;height:auto;", "}"], o.Y[16], e => {
            let {$isWelcomePage: i} = e;
            return i && (0,
            c.AH)(["align-items:center;"])
        }
        , r.Z.contentSidePaddingsNegative.val, r.Z.contentSidePaddingsNegative.val, ec.CR8, ec.CR8, e => {
            let {$isWelcomePage: i} = e;
            return !i && (0,
            c.AH)(["left:-", ";right:-", ";"], o.Y[24], o.Y[24])
        }
        )
          , ef = c.Ay.div.withConfig({
            componentId: "sc-b8c78cd-2"
        })(["overflow:hidden;height:50px;", ""], e => {
            let {$isWelcomePage: i} = e;
            return i && (0,
            c.AH)(["display:flex;align-items:center;"])
        }
        )
          , eS = c.Ay.strong.withConfig({
            componentId: "sc-b8c78cd-3"
        })(["color:", ";"], ec.gQ5)
          , eT = c.Ay.span.withConfig({
            componentId: "sc-b8c78cd-4"
        })(["", ";line-height:1;display:inline-block;padding:4px ", ";margin-right:", ";border-radius:12px;color:", ";", ""], eg.C, o.Y[8], o.Y[16], ec.U6f, e => {
            let {type: i} = e;
            return "product" === i ? (0,
            c.AH)(["background-color:", ";"], ec.NMo) : "community" === i || "search" === i ? (0,
            c.AH)(["background-color:", ";"], ec.M5_) : void 0
        }
        )
          , ey = T.e.DEFAULT
          , eb = e => {
            let i = e || "";
            return i = i.length > 60 ? `${i.slice(0, 60)}...` : i,
            (0,
            t.jsx)("strong", {
                children: i
            })
        }
          , ek = (e, i, n, a) => e && (ey === T.e.BIG || n === T.e.BIG) ? (0,
        t.jsx)(ed, {
            src: e,
            alt: i || "",
            isLeadingImage: a
        }) : null
          , eI = e => e ? (0,
        t.jsx)(ea.z, {
            score: e,
            size: 12
        }) : ""
          , eA = e => (0,
        t.jsx)("strong", {
            children: e
        })
          , ex = e => (0,
        t.jsx)("strong", {
            children: e
        })
          , eP = e => e ? (0,
        t.jsx)(eS, {
            children: (0,
            t.jsx)(et.gm, {
                price: e
            })
        }) : ""
          , eE = (e, i) => {
            let n = i && e ? i.replace(e, "") : null;
            return n ? (0,
            t.jsxs)(t.Fragment, {
                children: [(0,
                t.jsx)("strong", {
                    children: e
                }), " ", n]
            }) : i || ""
        }
          , eC = e => (0,
        t.jsx)("strong", {
            children: e
        })
          , ew = (e, i) => i(B, e.userName || "", e.cityName || "", eE(e.brandName, e.fullProductName), eI(e.rating))
          , eU = (e, i) => i(H, e.userName || "", e.cityName || "", (0,
        t.jsxs)(t.Fragment, {
            children: [" ", (0,
            t.jsx)(Q.A, {
                width: 12,
                height: 12
            }), " ", (0,
            t.jsx)("strong", {
                children: i(q)
            }), " "]
        }))
          , eR = (e, i) => i(R, e.userName || "", e.cityName || "", (0,
        t.jsxs)(t.Fragment, {
            children: [ek(e.imageUrl, e.fullProductName), eE(e.brandName, e.fullProductName)]
        }), eP(e.displayPrice) || "")
          , eO = (e, i) => i(Z, e.userName || "", (0,
        t.jsxs)(t.Fragment, {
            children: [ek(e.imageUrl, e.fullProductName), eE(e.brandName, e.fullProductName)]
        }), eP(e.displayPrice) || "", e.cityName || "")
          , ej = (e, i) => i(w, (0,
        t.jsxs)(t.Fragment, {
            children: [ek(e.imageUrl, e.fullProductName, void 0, !0), eE(e.brandName, e.fullProductName)]
        }), eP(e.displayPrice) || "", e.userName || "", e.cityName || "")
          , eD = (e, i) => i(j, e.userName || "", e.cityName || "", (0,
        t.jsxs)(t.Fragment, {
            children: [ek(e.imageUrl, e.fullProductName), eE(e.brandName, e.fullProductName)]
        }), eP(e.displayPrice) || "")
          , eF = (e, i) => i(C, e.userName || "", e.cityName || "", (0,
        t.jsxs)(t.Fragment, {
            children: [" ", (0,
            t.jsx)(K.A, {
                width: 12,
                height: 12
            }), " ", eC(e.searchString), " "]
        }))
          , eH = (e, i) => i(O, e.userName || "", e.cityName || "", e.oAuthProviderName || "")
          , ez = (e, i) => i($, e.userName || "", e.targetUserName || "", eZ(e.voteTypeId, i), eb(e.quote))
          , e$ = (e, i) => i(Y, e.userName || "", (0,
        t.jsx)(X.A, {
            width: 12,
            height: 12
        }), e.targetUserName || "", eb(e.quote))
          , eL = (e, i) => i(P, e.userName || "", (0,
        t.jsx)(ee, {
            width: 12,
            height: 12
        }), (0,
        t.jsxs)(t.Fragment, {
            children: [ek(e.imageUrl, e.fullProductName), eE(e.brandName, e.fullProductName)]
        }), eb(e.quote))
          , eM = (e, i) => i(z, e.userName || "", (0,
        t.jsx)(X.A, {
            width: 12,
            height: 12
        }), e.targetUserName || "", (0,
        t.jsx)(ee, {
            width: 12,
            height: 12
        }), eb(e.quote))
          , eW = (e, i) => i(V, e.userName || "", e.targetUserName || "", (0,
        t.jsx)(ee, {
            width: 12,
            height: 12
        }), eZ(e.voteTypeId, i), eb(e.quote))
          , eq = (e, i) => i(E, e.userName || "", e.targetUserName || "", eZ(e.voteTypeId, i), eb(e.quote))
          , eG = (e, i) => i(D, e.userName || "", e.targetUserName || "", (0,
        t.jsx)(X.A, {
            width: 12,
            height: 12
        }), eb(e.quote))
          , eB = (e, i) => i(F, e.userName || "", (0,
        t.jsx)(ei.A, {
            width: 12,
            height: 12
        }), e.brandName ? eA(e.brandName) : ex(e.productTypeName), eb(e.quote))
          , eV = (e, i) => i(M, e.userName || "", e.brandName ? eA(e.brandName) : ex(e.productTypeName), eb(e.quote))
          , e_ = (e, i) => i(L, e.userName || "", e.targetUserName || "", (0,
        t.jsx)(X.A, {
            width: 12,
            height: 12
        }), eZ(e.voteTypeId, i), eb(e.quote))
          , eY = (e, i) => i(U, e.userName || "", "", eA(e.brandName))
          , eZ = (e, i) => {
            switch (e) {
            case 1:
                return i(G);
            case 2:
                return i(W);
            case 3:
                return i(_);
            default:
                return ""
            }
        }
          , eQ = e => {
            let {item: i, size: n} = e
              , {__: a} = (0,
            er.W)();
            ey = n;
            let r = (0,
            u.useMemo)( () => {
                switch (i.socialShoppingTransactionTypeId) {
                case A.RATING:
                    return ew(i, a);
                case A.USERCREATION:
                    return eU(i, a);
                case A.ORDEREDPRODUCT:
                    return eR(i, a);
                case A.PICKEDUPPRODUCT:
                    return eO(i, a);
                case A.SHIPPEDPRODUCT:
                    return ej(i, a);
                case A.WATCHEDPRODUCT:
                    return eD(i, a);
                case A.SEARCH:
                    return eF(i, a);
                case A.OAUTHUSERCONNECTED:
                    return eH(i, a);
                case A.USERVOTESONRATING:
                    return ez(i, a);
                case A.USERCOMMENTSONRATING:
                    return e$(i, a);
                case A.USERASKSQUESTION:
                    return eL(i, a);
                case A.USERANSWERSQUESTION:
                    return eM(i, a);
                case A.USERVOTESONQUESTION:
                    return eW(i, a);
                case A.USERVOTESONANSWER:
                    return eq(i, a);
                case A.USERCOMMENTSONANSWER:
                    return eG(i, a);
                case A.USERSTARTSDISCUSSION:
                    return eB(i, a);
                case A.USERCONTRIBUTESTODISCUSSION:
                    return eV(i, a);
                case A.USERVOTESONDISCUSSIONPOST:
                    return e_(i, a);
                case A.USERLIKESBRAND:
                    return eY(i, a);
                default:
                    return en.vF.warn(`Unknown SocialShoppingItemId ${i.socialShoppingTransactionTypeId}`),
                    null
                }
            }
            , [i, a]);
            return (0,
            t.jsx)(t.Fragment, {
                children: r
            })
        }
          , eK = (e, i) => (e.url && e.url.startsWith(`/${i}`) ? e.url : `/${i}${e.url}`).toLowerCase()
          , eX = e => {
            let {item: i, position: n, isWelcomePage: a=!1, size: r=T.e.DEFAULT, ...l} = e
              , o = x(i.socialShoppingTransactionTypeId)
              , s = I(o)
              , {language: d, locale: c, timeZone: u} = (0,
            S.B2)()
              , m = Intl.DateTimeFormat().resolvedOptions().timeZone ?? u;
            return (0,
            t.jsx)(ev, {
                $size: r,
                $position: n,
                ...l,
                children: (0,
                t.jsxs)(eN, {
                    $isWelcomePage: a,
                    children: [(0,
                    t.jsx)("div", {
                        children: (0,
                        t.jsx)(eT, {
                            type: o,
                            children: (0,
                            S.fU)(i.dateTime, c, m)
                        })
                    }), (0,
                    t.jsx)(ef, {
                        $isWelcomePage: a,
                        children: (0,
                        t.jsx)("span", {
                            children: (0,
                            t.jsx)(eQ, {
                                item: i,
                                size: r
                            })
                        })
                    }), [A.USERCREATION].includes(i.socialShoppingTransactionTypeId) ? null : (0,
                    t.jsx)(f.R, {
                        href: eK(i, d),
                        onClick: s,
                        "aria-label": i.fullProductName || ""
                    })]
                })
            })
        }
          , eJ = e => {
            let {items: i, isWelcomePage: n, itemSize: a} = e
              , r = k();
            return (0,
            t.jsx)(e0, {
                ref: r,
                children: i.map( (e, i) => (0,
                t.jsx)(eX, {
                    size: a,
                    item: e,
                    position: i,
                    isWelcomePage: n
                }, `item-${e.userName}:${e.dateTime}:${e.socialShoppingTransactionTypeId}`))
            })
        }
          , e0 = c.Ay.div.withConfig({
            componentId: "sc-a59bad7f-0"
        })(["position:relative;height:calc((50px + ", " * 2) * 4 + 4px);"], o.Y[16])
          , e1 = {
            amountInclusive: 59.77,
            amountExclusive: 59.77,
            currency: n(21791).Sj.CHF
        }
          , e2 = [{
            brandName: "Siku",
            cityName: "Bern",
            dateTime: "2021-10-15T09:02:26.0798000Z",
            fullProductName: " Volvo Dumper",
            id: "1827925984",
            imageUrl: "//static.digitecgalaxus.ch/Files/2/0/3/0/0/0/8/9/siku-volvo-dumper-truck.jpg?fit=inside%7C160:128",
            oAuthProviderName: null,
            productTypeName: null,
            quote: "Gut",
            rating: 5,
            displayPrice: e1,
            searchString: null,
            socialShoppingTransactionTypeId: 1,
            targetUserName: null,
            url: "/ProductRating/3111805?sector=5",
            userName: ";))))",
            voteTypeId: null
        }, {
            brandName: null,
            cityName: "Basel",
            dateTime: "2021-10-15T08:28:35.4820000Z",
            fullProductName: null,
            id: "1517352390",
            imageUrl: null,
            oAuthProviderName: null,
            productTypeName: null,
            quote: null,
            rating: null,
            displayPrice: null,
            searchString: null,
            socialShoppingTransactionTypeId: 2,
            targetUserName: null,
            url: "",
            userName: "M.",
            voteTypeId: null
        }, {
            brandName: "Birkenstock",
            cityName: "Aeschi",
            dateTime: "2021-10-14T09:44:34.4217000Z",
            fullProductName: " Arizona Birko-Flor Normal",
            id: "86161474",
            imageUrl: "//static.digitecgalaxus.ch/Files/1/3/2/2/2/2/0/3/51791.jpg?fit=inside%7C160:128",
            oAuthProviderName: null,
            productTypeName: null,
            quote: null,
            rating: null,
            displayPrice: e1,
            searchString: null,
            socialShoppingTransactionTypeId: 3,
            targetUserName: null,
            url: "/s8/product/birkenstock-arizona-birko-flor-normal-41-hausschuhe-6002061",
            userName: "F.",
            voteTypeId: null
        }, {
            brandName: "Koenig",
            cityName: "Z\xfcrich",
            dateTime: "2021-10-15T08:28:30.9680000Z",
            fullProductName: " Raclette-Grill",
            id: "1728317572",
            imageUrl: "//static.digitecgalaxus.ch/Files/4/9/7/0/0/5/9/B02156_Raclette-Grill_591x428.jpg?fit=inside%7C160:128",
            oAuthProviderName: null,
            productTypeName: null,
            quote: null,
            rating: null,
            displayPrice: e1,
            searchString: null,
            socialShoppingTransactionTypeId: 4,
            targetUserName: null,
            url: "/s2/product/koenig-raclette-grill-racletteofen-238433",
            userName: "P.",
            voteTypeId: null
        }, {
            brandName: "Maxpedition",
            cityName: "Regensdorf ",
            dateTime: "2021-10-15T08:28:50.9245000Z",
            fullProductName: " CP-L Funkger\xe4t Holster",
            id: "1818665975",
            imageUrl: "//static.digitecgalaxus.ch/Files/4/7/7/0/6/6/9/4/301039-1.jpg?fit=inside%7C160:128",
            oAuthProviderName: null,
            productTypeName: null,
            quote: null,
            rating: null,
            displayPrice: e1,
            searchString: null,
            socialShoppingTransactionTypeId: 5,
            targetUserName: null,
            url: "/s3/product/maxpedition-cp-l-funkgeraet-holster-survival-tools-16334326",
            userName: "L.",
            voteTypeId: null
        }, void 0, void 0, void 0, {
            brandName: "TP-Link",
            cityName: null,
            dateTime: "2021-10-15T08:47:08.5960000Z",
            fullProductName: " EAP225-Outdoor: WLAN-AC AP",
            id: "1507830023",
            imageUrl: "//static.digitecgalaxus.ch/Files/2/7/0/3/6/4/2/5/null-H-003.xxl3.jpg?fit=inside%7C160:128",
            oAuthProviderName: null,
            productTypeName: null,
            quote: "Easy to install, but you need two RJ45 cables to complete installation",
            rating: null,
            displayPrice: e1,
            searchString: null,
            socialShoppingTransactionTypeId: 9,
            targetUserName: "philsnerurswell",
            url: "/ProductRating/2625478?sector=1",
            userName: "pbuchi2000",
            voteTypeId: 1
        }, void 0, {
            brandName: "Withings",
            cityName: null,
            dateTime: "2021-10-15T08:54:34.8599000Z",
            fullProductName: " ScanWatch",
            id: "1255484246",
            imageUrl: "//static.digitecgalaxus.ch/Files/3/7/5/1/1/3/3/1/1.jpg?fit=inside%7C160:128",
            oAuthProviderName: null,
            productTypeName: null,
            quote: "Withings ScanWatch vs. Samsung Galaxy Watch4 Classic?\nZu welcher w\xfcrdet ihr greifen und wieso? ",
            rating: null,
            displayPrice: e1,
            searchString: null,
            socialShoppingTransactionTypeId: 11,
            targetUserName: null,
            url: "/QuestionAndAnswer/430660?sector=1",
            userName: "dpetitpierre77",
            voteTypeId: null
        }, {
            brandName: "Red Cycling Products",
            cityName: null,
            dateTime: "2021-10-15T08:48:38.7089000Z",
            fullProductName: " Travelrack Light Gep\xe4cktr\xe4ger",
            id: "522232670",
            imageUrl: "//static.digitecgalaxus.ch/Files/9/7/0/8/9/3/3/Red_Cycling_Products_Travelrack_Light_Gepaecktraeger_01.jpg?fit=inside%7C160:128",
            oAuthProviderName: null,
            productTypeName: null,
            quote: "Hallo\nDas kannst Du, solange du genug Platz an der Sattelstange zur Verf\xfcgung hast. Habe ich selb...",
            rating: null,
            displayPrice: e1,
            searchString: null,
            socialShoppingTransactionTypeId: 12,
            targetUserName: "AnastasiaO",
            url: "/QuestionAndAnswer/430417?sector=3",
            userName: "Melina09Fabio11",
            voteTypeId: null
        }, {
            brandName: "Apple",
            cityName: null,
            dateTime: "2021-10-15T08:49:55.3754000Z",
            fullProductName: " AirPods Pro",
            id: "1712846664",
            imageUrl: "//static.digitecgalaxus.ch/Files/3/0/8/0/4/9/7/2/MWP22?fit=inside%7C160:128",
            oAuthProviderName: null,
            productTypeName: null,
            quote: 'Hauptsache man hat diese angeschrieben mit "Am Lager beim Lieferanten" und z\xe4hlt dann die Tage ru...',
            rating: null,
            displayPrice: e1,
            searchString: null,
            socialShoppingTransactionTypeId: 13,
            targetUserName: "alain.schneeberger",
            url: "/QuestionAndAnswer/202695?sector=1",
            userName: "cymatic22",
            voteTypeId: 1
        }, {
            brandName: "Sveltus",
            cityName: null,
            dateTime: "2021-10-15T08:58:00.2404000Z",
            fullProductName: " Power Bag",
            id: "12089135",
            imageUrl: "//static.digitecgalaxus.ch/Files/3/5/3/0/5/9/5/7/adjustable-power-bag.jpg?fit=inside%7C160:128",
            oAuthProviderName: null,
            productTypeName: null,
            quote: "Gem\xe4ss unserer Informationen des Lieferanten ist der Sack nicht bef\xfcllt.",
            rating: null,
            displayPrice: e1,
            searchString: null,
            socialShoppingTransactionTypeId: 14,
            targetUserName: "Raffaele.Santuari",
            url: "/QuestionAndAnswer/429513?sector=3",
            userName: "SebastianB148",
            voteTypeId: 1
        }, {
            brandName: "Xplora",
            cityName: null,
            dateTime: "2021-10-15T08:55:41.8260000Z",
            fullProductName: " X5 Play",
            id: "177153736",
            imageUrl: "//static.digitecgalaxus.ch/Files/3/9/7/6/9/6/3/1/X5_Play_Pink_Anruf.jpg?fit=inside%7C160:128",
            oAuthProviderName: null,
            productTypeName: null,
            quote: "Problem gel\xf6st. Bitte meinen Post oben l\xf6schen. mfg",
            rating: null,
            displayPrice: e1,
            searchString: null,
            socialShoppingTransactionTypeId: 15,
            targetUserName: "Fabian.Zaugg",
            url: "/QuestionAndAnswer/345939?sector=1",
            userName: "flomo",
            voteTypeId: null
        }, void 0, void 0, void 0, void 0].filter(e => void 0 !== e)
          , e3 = function() {
            let e = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : 1
              , i = Math.floor(Math.random() * e2.length);
            return e2.splice(i, e)
        };
        var e4 = n(18673)
          , e5 = n(35664);
        let e8 = e => {
            let {size: i=T.e.DEFAULT} = e
              , n = [];
            for (let e = 0; e < 4; e++) {
                let a = (0,
                t.jsxs)(e6, {
                    $size: i,
                    children: [(0,
                    t.jsx)(e7, {
                        children: "10:00"
                    }), (0,
                    t.jsxs)(e9, {
                        children: [(0,
                        t.jsx)(ie, {
                            $size: i
                        }), (0,
                        t.jsx)(ie, {
                            $size: i
                        })]
                    })]
                }, e);
                n.push(a)
            }
            return (0,
            t.jsx)(e0, {
                children: n
            })
        }
          , e6 = c.Ay.div.withConfig({
            componentId: "sc-f0bc375-0"
        })(["border-bottom:1px solid ", ";", ";padding:", " 0;display:flex;", ";&:last-child{border-bottom:1px solid transparent;}"], ec.CR8, e => {
            let {$size: i} = e;
            return i ? (0,
            c.AH)(["line-height:1.8;"]) : (0,
            c.AH)(["line-height:1.6;"])
        }
        , o.Y[16], e => {
            let {$size: i} = e;
            return i === T.e.BIG ? (0,
            c.AH)(["align-items:center;"]) : (0,
            c.AH)(["align-items:flex-start;"])
        }
        )
          , e7 = (0,
        c.Ay)(e4.E).withConfig({
            componentId: "sc-f0bc375-1"
        })(["line-height:1;padding:", " ", ";border-radius:12px;margin-right:", ";color:transparent;"], o.Y[4], o.Y[8], o.Y[16])
          , e9 = c.Ay.div.withConfig({
            componentId: "sc-f0bc375-2"
        })(["flex:1;height:50px;"])
          , ie = (0,
        c.Ay)(e5.r5).withConfig({
            componentId: "sc-f0bc375-3"
        })(["line-height:inherit;", ""], e => {
            let {$size: i} = e;
            return i === T.e.BIG && (0,
            c.AH)(["", "{", ";}"], l.s.desktopWidescreen, eg.FN)
        }
        )
          , ii = e => {
            let {isWelcomePage: i=!1, itemSize: n} = e
              , {environment: r, displayName: l} = (0,
            d.UK)()
              , {ref: o, inView: s} = (0,
            m.Wx)()
              , [c,N] = (0,
            u.useState)(null)
              , f = r === a.OH.PROD
              , {data: S, loading: y, error: b} = function(e) {
                let i = {
                    ...h,
                    ...e
                };
                return p.IT(g.GET_SOCIAL_SHOPPINGS, i)
            }({
                variables: {
                    take: 6,
                    latest: null
                },
                ssr: !1,
                pollInterval: 3e4,
                skip: !s
            });
            (0,
            u.useEffect)( () => {
                S?.socialShopping && N(v([...S.socialShopping.items], c || []))
            }
            , [S]),
            (0,
            u.useEffect)( () => {
                if (!f && c) {
                    let e = setInterval( () => {
                        N([...e3(Math.random() > .5 ? 6 : Math.floor(6 * Math.random())), ...c || []])
                    }
                    , 3e4);
                    return () => {
                        clearInterval(e)
                    }
                }
            }
            , [c, N]);
            let k = n === T.e.BIG ? ir : ia;
            return (0,
            t.jsx)(it, {
                ref: o,
                $isWelcomePage: i,
                children: (0,
                t.jsxs)(k, {
                    $isWelcomePage: i,
                    children: [(0,
                    t.jsxs)(il, {
                        tag: "h2",
                        $isWelcomePage: i,
                        children: [l, " Live"]
                    }), b ? null : y || !c ? (0,
                    t.jsx)(e8, {
                        size: n
                    }) : (0,
                    t.jsx)(eJ, {
                        items: c,
                        itemSize: n,
                        isWelcomePage: i
                    })]
                })
            })
        }
          , it = c.Ay.div.withConfig({
            componentId: "sc-d594ff04-0"
        })(["overflow-y:hidden;", ""], e => {
            let {$isWelcomePage: i} = e;
            return i ? (0,
            c.AH)(["margin-left:", ";margin-right:", ";", "{display:none;}"], r.Z.contentSidePaddingsNegative.val, r.Z.contentSidePaddingsNegative.val, l.s.mobileTablet) : (0,
            c.AH)(["margin-bottom:-", ";"], o.Y[16])
        }
        )
          , ia = c.Ay.div.withConfig({
            componentId: "sc-d594ff04-1"
        })(["overflow:hidden;padding:0 ", " 0;", "{padding:0 ", " 0;}"], o.Y[16], l.s.desktopWidescreen, o.Y[24])
          , ir = c.Ay.div.withConfig({
            componentId: "sc-d594ff04-2"
        })(["overflow-y:hidden;padding-left:", ";padding-right:", ";"], r.Z.contentSidePaddings.val, r.Z.contentSidePaddings.val)
          , il = (0,
        c.Ay)(s.Dl).withConfig({
            componentId: "sc-d594ff04-3"
        })(["margin-bottom:", ";", ""], o.Y[24], e => {
            let {$isWelcomePage: i} = e;
            if (i)
                return (0,
                c.AH)(["", "{margin-bottom:", ";}"], l.s.desktopWidescreen, o.Y[32])
        }
        )
          , io = ii
    }
    ,
    74530: (e, i, n) => {
        "use strict";
        n.d(i, {
            e: () => t
        });
        var t = function(e) {
            return e.BIG = "itemBig",
            e.DEFAULT = "itemDefault",
            e
        }({})
    }
    ,
    15733: (e, i, n) => {
        "use strict";
        n.d(i, {
            z: () => f
        });
        var t = n(31085)
          , a = n(90624)
          , r = n(41002);
        let l = [e => (0,
        a.LS)(e.count, 0, r.de, {
            0: "Noch keine Bewertung",
            one: (0,
            a.ai)("de-CH", e.count, 0) + " Bewertung",
            other: (0,
            a.ai)("de-CH", e.count, 0) + " Bewertungen"
        }), e => (0,
        a.LS)(e.count, 0, r.en, {
            0: "No ratings yet",
            one: (0,
            a.ai)("en-US", e.count, 0) + " rating",
            other: (0,
            a.ai)("en-US", e.count, 0) + " ratings"
        }), e => (0,
        a.LS)(e.count, 0, r.fr, {
            0: "Pas encore d'\xe9valuation",
            one: (0,
            a.ai)("fr-CH", e.count, 0) + " \xe9valuation",
            other: (0,
            a.ai)("fr-CH", e.count, 0) + " \xe9valuations"
        }), e => (0,
        a.LS)(e.count, 0, r.it, {
            0: "Ancora nessuna valutazione",
            one: (0,
            a.ai)("it-CH", e.count, 0) + " valutazione",
            other: (0,
            a.ai)("it-CH", e.count, 0) + " valutazioni"
        }), e => (0,
        a.LS)(e.count, 0, r.nl, {
            0: "nog geen beoordelingen",
            one: (0,
            a.ai)("nl-NL", e.count, 0) + " beoordeling",
            other: (0,
            a.ai)("nl-NL", e.count, 0) + " beoordelingen"
        })]
          , o = ["[0] von 5 Sternen", "[0] out of 5 stars", "[0] \xe9toiles sur 5", "[0] stelle su 5", "[0] van de 5 sterren"];
        var s = n(60766)
          , d = n(97867)
          , c = n(49960)
          , u = n(29757)
          , m = n(27066)
          , g = n(74064)
          , p = n(62088)
          , h = n(82402)
          , v = n.n(h)
          , N = n(54490);
        let f = e => {
            let {score: i=null, size: n=16, spacing: a=2, total: r=0, ariaLabel: s, totalType: d, onStarClick: c=u.lQ, onTotalClick: g=u.lQ, ...p} = e
              , {__icu: h, __: v} = (0,
            m.W)();
            if (null === i)
                return null;
            let b = r > 0;
            if (!b || void 0 === d)
                return (0,
                t.jsx)(y, {
                    ...p,
                    score: i,
                    spacing: a,
                    onClick: c,
                    size: n,
                    ariaLabel: s
                });
            let k = h(l, {
                count: r
            })
              , I = d === N.E.Long ? k : r
              , A = b && (0,
            t.jsx)(T, {
                "aria-hidden": !0,
                onClick: g,
                onKeyDown: e => {
                    "Enter" === e.key && g()
                }
                ,
                children: I
            })
              , x = !1 !== s ? {
                role: "img",
                "aria-label": s || `${k} ${v(o, i.toFixed(1))}`
            } : {
                "aria-hidden": !0
            };
            return (0,
            t.jsxs)(S, {
                ...x,
                ...p,
                children: [(0,
                t.jsx)(f, {
                    score: i,
                    size: n,
                    spacing: a,
                    total: 0,
                    ariaLabel: !1,
                    totalType: d,
                    onStarClick: c,
                    onTotalClick: g
                }), A]
            })
        }
          , S = g.Ay.span.withConfig({
            componentId: "sc-d4823a11-0"
        })(["display:flex;align-items:center;"])
          , T = g.Ay.span.withConfig({
            componentId: "sc-d4823a11-1"
        })(["margin-left:", ";color:", ";"], d.Y[8], p.bng)
          , y = e => {
            let {score: i, size: n, spacing: a, className: r, ariaLabel: l, ...d} = e
              , u = 0 === i
              , g = i >= 1 && i <= 5
              , {__: p} = (0,
            m.W)()
              , {brand: h} = (0,
            c.D)();
            u || g || (s.vF.error(`invalid value: ${i} for score prop, supported are 0 for empty case or numbers from 1 to 5`),
            i = 0);
            let N = Math.round(2 * i) / 2
              , f = N % 1 != 0
              , S = JSON.stringify(k[h].full.repeat(N) + (f ? k[h].left : ""))
              , T = JSON.stringify((f ? k[h].right : "") + k[h].full.repeat(5 - N))
              , y = !1 !== l ? {
                role: "img",
                "aria-label": l || p(o, i.toFixed(1))
            } : {
                "aria-hidden": !0
            };
            return (0,
            t.jsx)(I, {
                ...y,
                ...d,
                className: v().stars + (r ? ` ${r}` : ""),
                $fullStars: S,
                $emptyStars: T,
                $size: n,
                $spacing: a,
                $hasHalfStar: f
            })
        }
          , b = g.Ay.span.withConfig({
            componentId: "sc-d4823a11-2"
        })(["display:inline-block;white-space:nowrap;font-size:", ";line-height:1;letter-spacing:", ";margin-right:", ";&::before{margin-right:", ";}&::after{color:", ";}"], e => {
            let {$size: i} = e;
            return i ? `${i}px` : "inherit"
        }
        , e => {
            let {$spacing: i} = e;
            return `${i}px`
        }
        , e => {
            let {$spacing: i} = e;
            return `-${i}px`
        }
        , e => {
            let {$hasHalfStar: i, $spacing: n} = e;
            return i ? `-${n}px` : 0
        }
        , p.XLd)
          , k = {
            galaxus: {
                full: "⭑",
                left: "⯨",
                right: "⯩"
            },
            digitec: {
                full: "⭒",
                left: "⯫",
                right: "⯪"
            }
        }
          , I = (0,
        g.Ay)(b).withConfig({
            componentId: "sc-d4823a11-3"
        })(["&::before{content:", ";}&::after{content:", ";}"], e => {
            let {$fullStars: i} = e;
            return i
        }
        , e => {
            let {$emptyStars: i} = e;
            return i
        }
        )
    }
    ,
    54490: (e, i, n) => {
        "use strict";
        n.d(i, {
            E: () => t
        });
        var t = function(e) {
            return e[e.Short = 0] = "Short",
            e[e.Long = 1] = "Long",
            e
        }({})
    }
}]);
//# sourceMappingURL=/_next/static/sourcemaps/static/chunks/socialShoppingList.f913307ed3bbbcde.js.map
