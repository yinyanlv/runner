$(function () {

    var topicEditor = {
        init: function () {
            var self = this;

            self.initPlugins();
            self.initElements();
            self.initEvents();
        },

        initPlugins: function () {
            var self = this;
            var editor = new Editor();

            editor.render($('.editor')[0]);
        },

        initElements: function () {
            var self = this;


        },

        initEvents: function () {
            var self = this;


        }
    };

    topicEditor.init();
});