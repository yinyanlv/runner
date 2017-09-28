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
        },

        initElements: function () {
            var self = this;

            self.$btnSubmit = $('#btn-submit');
            self.$category = $('#category');
            self.$title = $('#title');
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

                var params = self.getValues();

                console.log(params);
            }
        },

        checkValid: function () {
            var self = this;
            var isValid = true;

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
                category: $.trim(self.$category.val()),
                title: $.trim(self.$title.val()),
                content: $.trim(self.editor.codemirror.getValue())
            };
        }
    };

    topicEditor.init();
});