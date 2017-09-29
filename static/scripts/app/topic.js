$(function () {

    var topic = {
        init: function () {
            var self = this;

            self.initPlugins();
            self.initElements();
            self.initStore();
            self.initEvents();
        },

        initPlugins: function () {
            var self = this;

            self.replyTopicEditor = new Editor();

            hljs.initHighlightingOnLoad();

            $('.markdown-body pre code').each(function (index, item) {

                hljs.highlightBlock(item);
            });

            self.replyTopicEditor.render($('#reply-topic-editor')[0]);
        },

        initElements: function () {
            var self = this;

            self.$inputTopicId = $('#topic-id');
            self.$inputUserId = $('#user-id');
            self.$btnEditTopic = $('#edit-topic');
            self.$btnDeleteTopic = $('#delete-topic');
            self.$btnReplyTopic = $('#btn-reply-topic');
        },

        initStore: function() {

            var self = this;

            self.store = {};

            self.store.topicId = $.trim(self.$inputTopicId.val());
            self.store.userId = $.trim(self.$inputUserId.val());
        },

        initEvents: function () {
            var self = this;

            self.$btnEditTopic.on('click', function () {

                window.location.href = globalConfig.path + '/edit-topic/' + self.store.topicId;
            });

            self.$btnDeleteTopic.on('click', function () {

                $.ajax({
                    url: globalConfig.path + '/delete-topic/' + self.store.topicId,
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

            self.$btnReplyTopic.on('click', function () {

                self.replyTopic();
            });
        },
        replyTopic: function () {
            var self = this;
            var content = $.trim(self.replyTopicEditor.codemirror.getValue());

            if (!content) {

                alert('回复内容不可为空');
                return;
            }

            if (self.$btnReplyTopic.is('disabled')) return;

            self.$btnReplyTopic.addClass('disabled');

            var params = {
                userId: self.store.userId,
                topicId: self.store.topicId,
                content: content
            };

            $.ajax({
                url: globalConfig.path + '/create-comment',
                type: 'POST',
                data: params,
                success: function (res) {

                    if (res.success) {

                        window.location.href = res.data;
                    } else {

                        alert(res.message);
                    }
                },
                complete: function () {
                    self.$btnReplyTopic.removeClass('disabled');
                }
            });
        }
    };

    topic.init();
});