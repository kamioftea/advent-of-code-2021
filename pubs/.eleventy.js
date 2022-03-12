const { EleventyRenderPlugin } = require("@11ty/eleventy");

module.exports = function(eleventyConfig) {
    eleventyConfig.addPlugin(EleventyRenderPlugin)

    // IntelliJ doesn't like frontmatter before <!doctype html> in root layout
    // So add the layout defaults here
    eleventyConfig.addGlobalData('title', 'Advent of Code 2021 | Jeff Horton')
    eleventyConfig.addGlobalData('layout', 'layout.njk')

    eleventyConfig.addPassthroughCopy('assets')

    return {
        passthroughFileCopy: true,
        markdownTemplateEngine: 'njk',
        pathPrefix: process.env.PATH_PREFIX ?? ''
    }
};
