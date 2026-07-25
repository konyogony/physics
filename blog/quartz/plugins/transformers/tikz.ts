import { Code, Root as MdRoot } from "mdast"
import { QuartzTransformerPlugin } from "../types"
import { visit } from "unist-util-visit"
import { load, tex, dvi2svg } from "node-tikzjax"
import { Argv } from "../../util/ctx"

async function tex2svg(input: string, argv: Argv) {
    await load()

    let content = input.trim()
    content = content
        .replace(/\\begin{document}/g, "")
        .replace(/\\end{document}/g, "")
        .replace(/\\documentclass(?:\[[^\]]*\])?\{[^}]*\}/g, "")
        .replace(/\\begin{tikzpicture}/g, "")
        .replace(/\\end{tikzpicture}/g, "")

    const fullInput = `\\begin{document}
\\begin{tikzpicture}
${content}
\\end{tikzpicture}
\\end{document}`

    const dvi = await tex(fullInput, {
        showConsole: true,
        tikzLibraries: "arrows.meta,positioning,calc",
    })

    return await dvi2svg(dvi)
}

interface TikzNode {
    index: number
    value: string
    parent: MdRoot
}

export const TikzJax: QuartzTransformerPlugin = () => {
    return {
        name: "TikzJax",
        markdownPlugins({ argv }) {
            return [
                () => async (tree: MdRoot, _file) => {
                    const nodes: TikzNode[] = []

                    visit(tree, "code", (node: Code, index, parent) => {
                        if (node.lang === "tikz" && parent && typeof index === "number") {
                            nodes.push({
                                index,
                                parent: parent as MdRoot,
                                value: node.value,
                            })
                        }
                    })

                    for (let i = 0; i < nodes.length; i++) {
                        const { index, parent, value } = nodes[i]
                        let svg = await tex2svg(value, argv)
                        svg = svg
                            .replaceAll(/("#000"|"black")/g, `"currentColor"`)
                            .replaceAll(/("#fff"|"white")/g, `"var(--background-primary)"`)
                        parent.children.splice(index, 1, {
                            type: "html",
                            value: `<div class="tikz">${svg}</div>`,
                        })
                    }
                },
            ]
        },
        externalResources() {
            return {
                css: [
                    {
                        content: "https://cdn.jsdelivr.net/npm/node-tikzjax@latest/css/fonts.css",
                    },
                ],
            }
        },
    }
}
