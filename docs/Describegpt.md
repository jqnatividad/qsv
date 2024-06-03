# `describegpt` command

`describegpt` allows users to infer extended metadata about a CSV dataset using large language models, in particular GPT chat completion models from OpenAI's API, Ollama, or an API compatible with the OpenAI API specification such as Jan. `describegpt` uses `qsv stats` and `qsv frequency` in the background to provide context to the model.

Note that this command uses LLMs for inferencing and is therefore prone to inaccurate information being produced. Verify output results before using them.

## QSV_OPENAI_KEY

`describegpt` requires an OpenAI API key to use by default. You can set this key using the `QSV_OPENAI_KEY` environment variable. Check [/docs/ENVIRONMENT_VARIABLES.md](/docs/ENVIRONMENT_VARIABLES.md) for more info.

If you're not using the OpenAI API, this environment variable is not necessary so long as you pass a value into `--api-key` (for example when using Ollama, use `--api-key ollama`).

## `--api-key <key>`

You can also specify your API key directly in your CLI using the `--api-key` option.

Note that if you already have `QSV_OPENAI_KEY` set as an environment variable and it is not empty, this environment variable will override your given flag.

If you're using Ollama, use `--api-key ollama`.

## `--json`

You can use the `--json` option to expect JSON output. This is useful for piping the output to other commands for example.

Note that **the `--json` option does not indicate to your prompt that you want to generate JSON output based on your dataset**. It instead ensures the command output is in JSON format. You must specify this within your prompts, such as adding the phrase "in JSON format" to your prompt.

If the prompt output is not in valid JSON format but the `--json` option is specified, the command will generate a default error JSON output printed to `stdout`, such as the following:

```json
{
    "option": {
        "error": "Invalid JSON output for option."
    }
}
```

You may often see this error when `--max-tokens` is set too low and therefore the output is incomplete.

The invalid output will be printed in `stderr`.

Note that `--json` may not be used alongside `--jsonl`, nor may they both be set to true in a prompt file at the same time. This will result in an error.

## `--jsonl`

Similar to `--json`, you can use the `--jsonl` option to expect [JSON Lines](https://jsonlines.org/) output.

If you use `--output` with `--jsonl`, the output will be written to a new file if it doesn't exist and any lines after the first will be appended to the file. If the file already exists, the output will be appended to the file. Each inference option (`--dictionary`, `--description`, `--tags`) will be written to a new line in the file.

If you use `--prompt-file` with `--jsonl`, the prompt name and timestamp will also be included in the JSONL output for each inference option.

Note that **the `--jsonl` option does not indicate to your prompt that you want to generate JSONL output based on your dataset**. It instead ensures the command output is in JSONL format. You must specify in your prompt to make a completion in JSON format, such as adding the phrase "in JSON format" to your prompt, and this will then be parsed into JSONL format by `describegpt`.

If the prompt output is not in valid JSON format but the `--jsonl` option is specified, the command will generate a default error JSON output printed to `stdout`, such as the following:

```json
{
    "option": {
        "error": "Invalid JSON output for option."
    }
}
```

You may often see this error when `--max-tokens` is set too low and therefore the output is incomplete.

The invalid output will be printed in `stderr`.

Note that `--jsonl` may not be used alongside `--json`, nor may they both be set to true in a prompt file at the same time. This will result in an error.

## `--max-tokens <value>`

`--max-tokens` is an option that allows you to specify the maximum number of tokens in the completion **output**. This is limited by the maximum number of tokens allowed by the model including the input tokens.

Input tokens may include the output of `qsv stats` and `qsv frequency` from your dataset, which can be large based on your dataset's size. Therefore we use `gpt-3.5-turbo-16k` as the default model for `describegpt` as it has a maximum token limit of 16,384.

It is highly recommended to set the `--max-tokens` option to set the maximum number of tokens in the completion output. Your output may be truncated if you set this value too low or you may receive errors depending on your options. The default is set to `50` as a safety measure.

## `--prompt-file`

With `describegpt` you can use a prompt file to add your own custom prompts and as an alternative to specifying certain options through the CLI. You can use the `--prompt-file` option to specify a prompt file to use.

If you do not specify a prompt file, default prompts will be used.

| Field                | Description                                                                                 |
| -------------------- | ------------------------------------------------------------------------------------------- |
| `name`               | The name of your prompt file.                                                               |
| `description`        | A description of your prompt file.                                                          |
| `author`             | Your name.                                                                                  |
| `version`            | The version of your prompt file.                                                            |
| `tokens`             | The maximum number of tokens in the completion output.                                      |
| `dictionary_prompt`  | The prompt for the `--dictionary` option.                                                   |
| `description_prompt` | The prompt for the `--description` option.                                                  |
| `tags_prompt`        | The prompt for the `--tags` option.                                                         |
| `json`               | Whether or not the output should be in JSON format (refer to [`--json`](#json) section).    |
| `jsonl`              | Whether or not the output should be in JSONL format (refer to [`--jsonl`](#jsonl) section). |

All fields must be present in your prompt file. If you do not want to use a certain prompt, you can set it to an empty string.

Within your prompts, you can use the following variables:

-   `{stats}`
-   `{frequency}`
-   `{json_add}`

These are replaced with the output of `qsv stats`, `qsv frequency` and conditionally ` (in JSON format)`. Note that `{json_add}` adds a space before `(in JSON format)`.

Here is an example of a prompt:

```json
{
    "name": "Sample prompt",
    "description": "A sample prompt file for describegpt.",
    "author": "qsv",
    "version": "1.0.0",
    "tokens": 50,
    "dictionary_prompt": "Here are the columns for each field in a data dictionary:\n\n- Type: the data type of this column\n- Label: a human-friendly label for this column\n- Description: a full description for this column (can be multiple sentences)\n\nGenerate a data dictionary as aforementioned{json_add} where each field has Name, Type, Label, and Description (so four columns in total) based on the following summary statistics and frequency data from a CSV file.\n\nSummary Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}",
    "description_prompt": "Generate only a description that is within 8 sentences about the entire dataset{json_add} based on the following summary statistics and frequency data derived from the CSV file it came from.\n\nSummary Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}\n\nDo not output the summary statistics for each field. Do not output the frequency for each field. Do not output data about each field individually, but instead output about the dataset as a whole in one 1-8 sentence description.",
    "tags_prompt": "A tag is a keyword or label that categorizes datasets with other, similar datasets. Using the right tags makes it easier for others to find and use datasets.\n\nGenerate single-word tags{json_add} about the dataset (lowercase only and remove all whitespace) based on the following summary statistics and frequency data from a CSV file.\n\nSummary Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}",
    "json": true,
    "jsonl": false,
    "base_url": "https://api.openai.com/v1",
    "ollama": false,
    "model": "gpt-3.5-turbo-16k",
    "timeout": 60
}
```

Note that this example has `tokens` set to `50` by default but you may want to increase this value to not result in errors as mentioned in the [`--json`](#json) & [`--max-tokens`](#max-tokens-value) section.

## `--ollama`

This is a required flag for when you're using the Ollama API, so make sure you pass it when using Ollama as your server. This is due to Ollama's API not being 1-1 compatible with the OpenAI API specification (particularly the `/models` endpoint may not be implemented yet, see https://github.com/ollama/ollama/issues/2430).

You may find the Ollama OpenAI compatibility documentation here: https://github.com/ollama/ollama/blob/main/docs/openai.md.

An example command for getting an inferred description is as follows:

```bash
qsv describegpt <filepath> --ollama --base-url http://localhost:11434 --api-key ollama --model <model> --max-tokens <number> --description
```

Remove the arrow brackets `<>` and replace `filepath` with your file's path, `<model>` with the model you want to use, and `number` with the max tokens you want to set based on your model's context size.
