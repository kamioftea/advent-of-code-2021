module.exports = function(eleventyConfig) {
    // IntelliJ doesn't like frontmatter before <!doctype html> in root layout
    // So add the layout defaults here
    eleventyConfig.addGlobalData('title', '')
    eleventyConfig.addGlobalData('layout', 'layout.njk')

    return {
        passthroughFileCopy: true,
        markdownTemplateEngine: 'njk',
        pathPrefix: process.env.PATH_PREFIX ?? ''
    }
};
