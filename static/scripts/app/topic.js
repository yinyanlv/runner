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
            self.$btnReplyTopic = $('#btn-reply-topic');
            self.$btnCollectTopic = $('#btn-collect-topic');
            self.$btnEditList = $('.btn-edit');
            self.$btnDeleteList = $('.btn-delete');
            self.$btnAggreeList = $('.btn-agree');
            self.$btnDisagreeList = $('.btn-disagree');
        },

        initStore: function() {

            var self = this;

            self.store = {};

            self.store.topicId = $.trim(self.$inputTopicId.val());
            self.store.userId = $.trim(self.$inputUserId.val());
        },

        initEvents: function () {
            var self = this;

            self.$btnReplyTopic.on('click', function () {

                self.replyTopic();
            });

            self.$btnCollectTopic.on('click', function () {
                self.collectTopic();
            });

            self.$btnEditList.on('click', function () {
                self.edit($(this));
            });

            self.$btnDeleteList.on('click', function () {

                self.delete($(this));
            });

            self.$btnAggreeList.on('click', function () {
                self.agree($(this));
            });

            self.$btnDisagreeList.on('click', function () {
                self.disagree($(this));
            });
        },

        edit: function ($btn) {
            var self = this;
            var isHandleTopic = $btn.closest('.operator').is('.operator-topic');

            if (isHandleTopic) {
                window.location.href = globalConfig.path + '/edit-topic/' + self.store.topicId;
            } else {

            }
        },

        delete: function ($btn) {
            var self = this;
            var isHandleTopic = $btn.closest('.operator').is('.operator-topic');
            var isConfirm, url;

            if (isHandleTopic) {
                isConfirm = confirm('您确定要删除该话题吗？');
                url = globalConfig.path + '/delete-topic/' + self.store.topicId;
            } else {
                isConfirm = confirm('您确定要删除该回复吗？');
            }

            if (isConfirm) {
                $.ajax({
                    url: url,
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
            }
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
        },

        collectTopic: function () {
            var self = this;
            var collectState = self.$btnCollectTopic.data('collected');
            var params = {
                userId: self.store.userId
            };

            if (collectState === true) {

                self.$btnCollectTopic.find('.fa').removeClass('fa-star').addClass('fa-star-o');
                self.$btnCollectTopic.find('.text').html('收藏');
                self.$btnCollectTopic.data('collected', false);

                $.ajax({
                    url: globalConfig.path + '/topic/cancel-collect/' + self.store.topicId,
                    type: 'POST',
                    data: params
                });
            } else {

                self.$btnCollectTopic.find('.fa').removeClass('fa-star-o').addClass('fa-star');
                self.$btnCollectTopic.find('.text').html('已收藏');
                self.$btnCollectTopic.data('collected', true);

                $.ajax({
                    url: globalConfig.path + '/topic/collect/' + self.store.topicId,
                    type: 'POST',
                    data: params
                });
            }
        },

        agree: function ($btn) {
            var self = this;
            var $num, curNum;
            var isHandleTopic = $btn.closest('.operator').is('.operator-topic');
            var agreeState = $btn.data('agreed');
            var params = {
                userId: self.store.userId
            };

            if (agreeState === true) {

                $btn.find('.fa').removeClass('fa-thumbs-up').addClass('fa-thumbs-o-up');
                $btn.find('.text').html('赞');
                $btn.data('agreed', false);
                $num = $btn.find('.num');
                curNum = parseInt($num.html());

                $num.html(curNum ? --curNum : 0);

                params.state = 0;

                $.ajax({
                    url: globalConfig.path + '/topic/vote/' + self.store.topicId,
                    type: 'POST',
                    data: params
                });
            } else {
                params.state = 1;

                $btn.find('.fa').removeClass('fa-thumbs-o-up').addClass('fa-thumbs-up');
                $btn.find('.text').html('已赞');
                $btn.data('agreed', true);
                $num = $btn.find('.num');
                curNum = parseInt($num.html());

                $num.html(++curNum);

                $.ajax({
                    url: globalConfig.path + '/topic/vote/' + self.store.topicId,
                    type: 'POST',
                    data: params
                });

                var $btnDisagree = $btn.closest('.operator').find('.btn-disagree');
                var disagreeState = $btnDisagree.data('disagreed');

                if (disagreeState === true) {
                    $btnDisagree.find('.fa').removeClass('fa-thumbs-down').addClass('fa-thumbs-o-down');
                    $btnDisagree.find('.text').html('踩');
                    $btnDisagree.data('disagreed', false);
                    $num = $btnDisagree.find('.num');
                    curNum = parseInt($num.html());

                    $num.html(curNum ? --curNum : 0);
                }
            }
        },

        disagree: function ($btn) {
            var self = this;
            var $num, curNum;
            var isHandleTopic = $btn.closest('.operator').is('.operator-topic');
            var disagreedState = $btn.data('disagreed');
            var params = {
                userId: self.store.userId
            };

            if (disagreedState === true) {

                params.state = 0;

                $btn.find('.fa').removeClass('fa-thumbs-down').addClass('fa-thumbs-o-down');
                $btn.find('.text').html('踩');
                $btn.data('disagreed', false);
                $num = $btn.find('.num');
                curNum = parseInt($num.html());

                $num.html(curNum ? --curNum : 0);

                params.state = 0;

                $.ajax({
                    url: globalConfig.path + '/topic/vote/' + self.store.topicId,
                    type: 'POST',
                    data: params
                });
            } else {

                params.state = -1;
                $btn.find('.fa').removeClass('fa-thumbs-o-down').addClass('fa-thumbs-down');
                $btn.find('.text').html('已踩');
                $btn.data('disagreed', true);
                $num = $btn.find('.num');
                curNum = parseInt($num.html());

                $num.html(++curNum);

                $.ajax({
                    url: globalConfig.path + '/topic/vote/' + self.store.topicId,
                    type: 'POST',
                    data: params
                });

                var $btnAgree = $btn.closest('.operator').find('.btn-agree');
                var agreeState = $btnAgree.data('agreed');

                if (agreeState === true) {
                    $btnAgree.find('.fa').removeClass('fa-thumbs-up').addClass('fa-thumbs-o-up');
                    $btnAgree.find('.text').html('赞');
                    $btnAgree.data('agreed', false);
                    $num = $btnAgree.find('.num');
                    curNum = parseInt($num.html());

                    $num.html(curNum ? --curNum : 0);
                }
            }
        }
    };

    topic.init();
});