$(function () {

    var topic = {
        init: function () {
            var self = this;

            self.initPlugins();
            self.initElements();
            self.initEvents();
        },

        initPlugins: function () {
            var self = this;
            var editor = new Editor();

            hljs.initHighlightingOnLoad();

            $('.markdown-body pre code').each(function (index, item) {

                hljs.highlightBlock(item);
            });

            editor.render($('.editor')[0]);
        },

        initElements: function () {
            var self = this;


        },

        initEvents: function () {
            var self = this;


        }
    };

    topic.init();
});