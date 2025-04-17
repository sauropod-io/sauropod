# Your first workflow

Lets make a workflow to compare species of animals.

## First task

First we make a task called "Wikipedia Grabber" with these instructions:

```
Give me the Wikipedia page for ${topic}.
```

Enable structured outputs and make one field:

- `page_content`: Text

Give the task permission to use the fetch tool or a tool like [markitdown](https://github.com/microsoft/markitdown).

## Second task

Now make a task named "Animal Classifier" and give it these instructions:

```
Is a ${animal1} like a ${animal2}?

Compare the Wikipedia pages for ${animal1}  and ${animal2} to make a determination.
```

Enable structured outputs and make two fields:

- `is_similar`: True/False
- `reason`: Text

Now give "Animal Classifier" permission to run "Wikipedia Grabber" by enabling it in the tools dropdown.

## Running the tasks

Once you've saved your animal classifier, click the Run button.

You can either run the task from the UI page, or use the code snippets in the other tabs to call it via API.

## Conclusion

In reality we could have done this all in one task, but this is a good way to get the general idea of how you can use tasks together to make bigger flows.
