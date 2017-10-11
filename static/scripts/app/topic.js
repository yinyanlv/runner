$(function () {

    var topic = {
        init: function () {
            var self = this;

            self.initElements();
            self.initStore();
            self.initPlugins();

            if (self.isLogin()) {
                self.initEvents();
            }
        },

        initPlugins: function () {
            var self = this;

            hljs.initHighlightingOnLoad();

            $('.markdown-body pre code').each(function (index, item) {

                hljs.highlightBlock(item);
            });

            if (self.isLogin()) {

                $('textarea.editor').each(function(){
                    var editor = new Editor();
                    var $this = $(this);

                    editor.render(this);

                    $this.data('editor', editor);
                    var $input = $(editor.codemirror.display.input);

                    $input.keydown(function(e){
                        if (e.keyCode === 13 && (e.ctrlKey || e.metaKey)) {
                            e.preventDefault();
                            $input.closest('.editor-wrapper').find('.btn-reply-topic').click();
                        }
                    });

                    var codeMirrorGoLineUp = CodeMirror.commands.goLineUp;
                    var codeMirrorGoLineDown = CodeMirror.commands.goLineDown;
                    var codeMirrorNewlineAndIndent = CodeMirror.commands.newlineAndIndent;

                    $input.atwho({
                        at: '@',
                        data: self.store.usernameList
                    })
                    .on('shown.atwho', function () {
                        CodeMirror.commands.goLineUp = $.noop;
                        CodeMirror.commands.goLineDown = $.noop;
                        CodeMirror.commands.newlineAndIndent = $.noop;
                    })
                    .on('hidden.atwho', function () {
                        CodeMirror.commands.goLineUp = codeMirrorGoLineUp;
                        CodeMirror.commands.goLineDown = codeMirrorGoLineDown;
                        CodeMirror.commands.newlineAndIndent = codeMirrorNewlineAndIndent;
                    });
                });
            }
        },

        initElements: function () {
            var self = this;

            self.$inputTopicId = $('#topic-id');
            self.$inputUserId = $('#user-id');
            self.$btnCollectTopic = $('#btn-collect-topic');
            self.$btnStickTopic = $('#btn-stick-topic');
            self.$btnEssenceTopic = $('#btn-essence-topic');
            self.$btnEditList = $('.btn-edit');
            self.$btnDeleteList = $('.btn-delete');
            self.$btnAggreeList = $('.btn-agree');
            self.$btnDisagreeList = $('.btn-disagree');
            self.$btnReplyList = $('.btn-reply');
            self.$btnReplyTopicList = $('.btn-reply-topic');
        },

        initStore: function () {

            var self = this;

            self.store = {};

            self.store.topicId = $.trim(self.$inputTopicId.val());
            self.store.userId = $.trim(self.$inputUserId.val());

            var tempList = [];

            $('.comment-info a.username').each(function(index, item) {
                var username = $(item).html();

                tempList.push(username);
            });

            self.store.usernameList = $.unique(tempList);
        },

        initEvents: function () {
            var self = this;

            self.$btnReplyTopicList.on('click', function () {

                self.replyTopic($(this));
            });

            self.$btnCollectTopic.on('click', function () {
                self.collectTopic();
            });

            self.$btnStickTopic.on('click', function () {
                self.stickTopic();
            });

            self.$btnEssenceTopic.on('click', function () {

                self.essenceTopic();
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

            self.$btnReplyList.on('click', function () {
                self.showReplyEditor($(this));
            });
        },

        edit: function ($btn) {
            var self = this;
            var isHandleTopic = $btn.closest('.operator').is('.operator-topic');

            if (isHandleTopic) {
                window.location.href = globalConfig.path + '/edit-topic/' + self.store.topicId;
            } else {

                var commentId = $btn.closest('.operator').data('comment-id');

                window.location.href = globalConfig.path + '/edit-comment/' + commentId;
            }
        },

        delete: function ($btn) {
            var self = this;
            var isHandleTopic = $btn.closest('.operator').is('.operator-topic');
            var isConfirm, url, params = null;

            if (isHandleTopic) {
                isConfirm = confirm('您确定要删除该话题吗？');
                url = globalConfig.path + '/delete-topic/' + self.store.topicId;
            } else {
                isConfirm = confirm('您确定要删除该回复吗？');
                var commentId = $btn.closest('.operator').data('comment-id');
                url = globalConfig.path + '/delete-comment/' + commentId;
                params = {
                    topicId: self.store.topicId
                };
            }

            if (isConfirm) {
                $.ajax({
                    url: url,
                    type: 'DELETE',
                    data: params,
                    success: function (res) {

                        if (res.success) {

                            window.location.href = res.data;
                        } else {

                            alert(res.message);
                        }
                    }
                });
            }
        },

        replyTopic: function ($btn) {
            var self = this;
            var editor = $btn.closest('.editor-wrapper').find('textarea.editor').data('editor');
            var content = $.trim(editor.codemirror.getValue());

            if (!content) {

                alert('回复内容不可为空');
                return;
            }

            if ($btn.is('.disabled')) return;

            $btn.addClass('disabled');

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
                    $btn.removeClass('disabled');
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

                params.isCollect = false;
            } else {

                self.$btnCollectTopic.find('.fa').removeClass('fa-star-o').addClass('fa-star');
                self.$btnCollectTopic.find('.text').html('已收藏');
                self.$btnCollectTopic.data('collected', true);

                params.isCollect = true;
            }

            $.ajax({
                url: globalConfig.path + '/topic/collect/' + self.store.topicId,
                type: 'POST',
                data: params
            });
        },

        stickTopic: function () {
            var self = this;
            var stickState = self.$btnStickTopic.data('sticked');
            var params = {};

            if (stickState === true) {

                self.$btnStickTopic.find('.text').html('置顶');
                self.$btnStickTopic.data('sticked', false);

                params.isSticked = false;
            } else {

                self.$btnStickTopic.find('.text').html('已置顶');
                self.$btnStickTopic.data('sticked', true);

                params.isSticked = true;
            }

            $.ajax({
                url: globalConfig.path + '/topic/stick/' + self.store.topicId,
                type: 'POST',
                data: params,
                success: function (res) {

                    if (res.success) {

                        window.location.reload();
                    } else {

                        alert(res.message);
                    }
                }
            });
        },

        essenceTopic: function () {
            var self = this;
            var essenceState = self.$btnEssenceTopic.data('essenced');
            var params = {};

            if (essenceState === true) {

                self.$btnEssenceTopic.find('.text').html('设为精华');
                self.$btnEssenceTopic.data('essenced', false);

                params.isEssenced = false;
            } else {

                self.$btnEssenceTopic.find('.text').html('已设为精华');
                self.$btnEssenceTopic.data('essenced', true);

                params.isEssenced = true;
            }

            $.ajax({
                url: globalConfig.path + '/topic/essence/' + self.store.topicId,
                type: 'POST',
                data: params,
                success: function (res) {

                    if (res.success) {

                        window.location.reload();
                    } else {

                        alert(res.message);
                    }
                }
            });
        },

        agree: function ($btn) {
            var self = this;
            var $num, curNum;
            var isHandleTopic = $btn.closest('.operator').is('.operator-topic');
            var agreeState = $btn.data('agreed');
            var url;
            var params = {
                userId: self.store.userId
            };

            if (isHandleTopic) {
                url = globalConfig.path + '/topic/vote/' + self.store.topicId;
            } else {
                var commentId = $btn.closest('.operator').data('comment-id');
                url = globalConfig.path + '/comment/vote/' + commentId;
            }

            if (agreeState === true) {

                $btn.find('.fa').removeClass('fa-thumbs-up').addClass('fa-thumbs-o-up');
                $btn.find('.text').html('赞');
                $btn.data('agreed', false);
                $num = $btn.find('.num');
                curNum = parseInt($num.html());

                $num.html(curNum ? --curNum : 0);

                params.state = 0;

            } else {
                params.state = 1;

                $btn.find('.fa').removeClass('fa-thumbs-o-up').addClass('fa-thumbs-up');
                $btn.find('.text').html('已赞');
                $btn.data('agreed', true);
                $num = $btn.find('.num');
                curNum = parseInt($num.html());

                $num.html(++curNum);

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


            $.ajax({
                url: url,
                type: 'POST',
                data: params
            });
        },

        disagree: function ($btn) {
            var self = this;
            var $num, curNum;
            var isHandleTopic = $btn.closest('.operator').is('.operator-topic');
            var disagreedState = $btn.data('disagreed');
            var url;
            var params = {
                userId: self.store.userId
            };

            if (isHandleTopic) {
                url = globalConfig.path + '/topic/vote/' + self.store.topicId;
            } else {
                var commentId = $btn.closest('.operator').data('comment-id');
                url = globalConfig.path + '/comment/vote/' + commentId;
            }

            if (disagreedState === true) {

                $btn.find('.fa').removeClass('fa-thumbs-down').addClass('fa-thumbs-o-down');
                $btn.find('.text').html('踩');
                $btn.data('disagreed', false);
                $num = $btn.find('.num');
                curNum = parseInt($num.html());

                $num.html(curNum ? --curNum : 0);

                params.state = 0;

            } else {

                params.state = -1;
                $btn.find('.fa').removeClass('fa-thumbs-o-down').addClass('fa-thumbs-down');
                $btn.find('.text').html('已踩');
                $btn.data('disagreed', true);
                $num = $btn.find('.num');
                curNum = parseInt($num.html());

                $num.html(++curNum);

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

            $.ajax({
                url: url,
                type: 'POST',
                data: params
            });
        },

        showReplyEditor: function ($btn) {
            var isEditorShowed = $btn.data('show-editor');
            var $editorBox = $btn.closest('.comment ').find('.editor-wrapper-box');
            var textarea = $editorBox.find('textarea.editor');
            var editor = textarea.data('editor');
            var username = textarea.data('username');
            var $text = $btn.find('.text');

            if (isEditorShowed) {
                $btn.data('show-editor', false);
                $text.html('回复');
                $editorBox.hide('fast');
            } else {
                $btn.data('show-editor', true);
                $text.html('收起回复');
                $editorBox.show('fast', function() {
                    var cm = editor.codemirror;
                    cm.focus();
                    if(cm.getValue().indexOf('@' + username) < 0){
                        editor.push('@' + username + ' ');
                    }
                });
            }
        },

        isLogin: function () {
            var self = this;

            return globalConfig.username === '' ? false : true;
        }
    };

    topic.init();
});