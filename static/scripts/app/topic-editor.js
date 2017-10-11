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

            self.editor = new Editor();

            self.editor.render($('.editor')[0]);

            var $input = $(self.editor.codemirror.display.input);

            $input.keydown(function(e){
                if (e.keyCode === 13 && (e.ctrlKey || e.metaKey)) {
                    e.preventDefault();

                    self.submit();
                }
            });
        },

        initElements: function () {
            var self = this;

            self.$btnSubmit = $('#btn-submit');
            self.$category = $('#category');
            self.$title = $('#title');
            self.$topicId = $('#topic-id')
        },

        initEvents: function () {
            var self = this;

            self.$btnSubmit.on('click', function () {

                self.submit();
            });
        },

        submit: function () {
            var self = this;

            if (self.checkValid()) {

                if (self.$btnSubmit.is('.disabled')) return;

                self.$btnSubmit.addClass('disabled');

                var params = self.getValues();
                var options = params.id ? {
                    url: globalConfig.path + '/edit-topic/' + params.id,
                    method: 'PUT'
                } : {
                    url: globalConfig.path + '/create-topic',
                    method: 'POST'
                };

                $.ajax({
                    url: options.url,
                    type: options.method,
                    data: params,
                    success: function (res) {

                        if (res.success) {

                            window.location.href = res.data;
                        } else {

                            alert(res.message);
                        }
                    },
                    complete: function () {
                        self.$btnSubmit.removeClass('disabled');
                    }
                });
            }
        },

        checkValid: function () {
            var self = this;
            var isValid = true;

            if (!$.trim(self.$category.find('option:selected').val())) {

                alert("版块不能为空！");

                isValid = false;
                return isValid;
            }

            if (!$.trim(self.$title.val())) {

                alert("话题标题不能为空！");

                isValid = false;
                return isValid;
            }

            if (!$.trim(self.editor.codemirror.getValue())) {

                alert("话题内容不能为空！");

                isValid = false;
                return isValid;
            }

            return isValid;
        },

        getValues: function () {
            var self = this;

            return {
                id: $.trim(self.$topicId.val()),
                category: $.trim(self.$category.find('option:selected').val()),
                title: $.trim(self.$title.val()),
                content: $.trim(self.editor.codemirror.getValue())
            };
        }
    };

    topicEditor.init();
});