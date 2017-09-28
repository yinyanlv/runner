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

            self.$btnEditTopic = $('#edit-topic');
            self.$btnDeleteTopic = $('#delete-topic');
        },

        initEvents: function () {
            var self = this;

            self.$btnEditTopic.on('click', function () {

                var topicId = $(this).data('topic-id');

                window.location.href = globalConfig.path + '/edit-topic/' + topicId;
            });

            self.$btnDeleteTopic.on('click', function () {

                var topicId = $(this).data('topic-id');

                $.ajax({
                    url: globalConfig.path + '/delete-topic/' + topicId,
                    type: 'DELETE',
                    success: function (res) {

                        if (res.success) {

                            alert(res.message);
                            window.location.href = res.data;
                        } else {

                            alert(res.message);
                        }
                    }
                });
            });
        }
    };

    topic.init();
});