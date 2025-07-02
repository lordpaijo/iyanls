use clap::Parser;
use owo_colors::OwoColorize;
use std::{fs, path::PathBuf, process::exit};

mod cli;
mod display;
mod file_ops;
mod sorting;
mod types;
mod utils;

use cli::Args;
use display::{export_json, print_table_from_files};
use file_ops::get_file;
use sorting::{get_sort_order, sort_files};
use std::os::unix::io::AsRawFd;
use types::FileEntry;

fn main() {
    let tty_available = unsafe { libc::isatty(std::io::stdout().as_raw_fd()) == 1 };
    let args = Args::parse();
    let sort_order = get_sort_order(&args);

    let path = args.path.unwrap_or(PathBuf::from("."));
    let timezone = utils::parse_timezone(&args.timezone);

    if let Ok(exists) = fs::exists(&path) {
        if exists {
            let mut files = get_file(
                &path,
    local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not (vim.uv or vim.loop).fs_stat(lazypath) then
	local lazyrepo = "https://github.com/folke/lazy.nvim.git"
	local out = vim.fn.system({ "git", "clone", "--filter=blob:none", "--branch=stable", lazyrepo, lazypath })
	if vim.v.shell_error ~= 0 then
		vim.api.nvim_echo({
			{ "Failed to clone lazy.nvim:\n", "ErrorMsg" },
			{ out, "WarningMsg" },
			{ "\nPress any key to exit..." },
		}, true, {})
		vim.fn.getchar()
		os.exit(1)
	end
end
vim.opt.rtp:prepend(lazypath)

local options = {
	tabstop = 4,
	shiftwidth = 4,
	expandtab = true,
	number = true,
	relativenumber = true,
	autoindent = true,
	showmode = false,
	title = true,
	swapfile = false,
	breakindent = true,
	hidden = true,
	undofile = true,
	autowriteall = true,
	termguicolors = true,
	splitbelow = true,
	scrolloff = 7,
	mouse = "a",
	encoding = "UTF-8",
	background = "dark",
	signcolumn = "yes",
}

for key, value in pairs(options) do
	vim.opt[key] = value
end

vim.g.mapleader = " "
vim.opt.undodir = os.getenv("HOME") .. "/.nvim_undodir/"
vim.opt.clipboard:append("unnamedplus")

-- for nvimtree
vim.g.loaded_netrw = 1
vim.g.loaded_netrwPlugin = 1
vim.g.compile_mode = { baleia_setup = true }

vim.cmd([[aunmenu PopUp.How-to\ disable\ mouse]])
vim.cmd([[aunmenu PopUp.-1-]])

vim.g.VM_maps = {
	["Find Under"] = "<C-d>",
	["Find Subword Under"] = "<C-d>",
	["I Return"] = "<S-CR>",
	["Add Cursor Down"] = "",
	["Add Cursor Up"] = "",
}

-- AUGRUP AND AUTOCMD
vim.api.nvim_create_augroup("highlight_yank", { clear = true })
vim.api.nvim_create_autocmd("TextYankPost", {
	group = "highlight_yank",
	pattern = "*",
	callback = function()
		vim.highlight.on_yank({ higroup = "IncSearch", timeout = 100 })
	end,
	desc = "Highlight yanked text",
})

vim.api.nvim_create_autocmd("FileType", {
	pattern = "php",
	callback = function()
		vim.opt.iskeyword:append("$")
	end,
	desc = "Add $ to iskeyword for PHP files",
})

vim.api.nvim_create_augroup("number_toggle", { clear = true })
vim.api.nvim_create_autocmd("InsertLeave", {
	group = "number_toggle",
	pattern = "*",
	callback = function()
		vim.opt.number = true
		vim.opt.relativenumber = true
	end,
	desc = "Enable number and relativenumber on insert leave",
})

vim.api.nvim_create_autocmd("InsertEnter", {
	group = "number_toggle",
	pattern = "*",
	callback = function()
		vim.opt.number = true
		vim.opt.relativenumber = false
	end,
	desc = "Enable number, disable relativenumber on insert enter",
})

-- Autosave on focus lost or buffer leave
vim.api.nvim_create_augroup("autosave", { clear = true })
vim.api.nvim_create_autocmd("FocusLost", {
	group = "autosave",
	pattern = "*",
	command = "wall!",
	desc = "Save all buffers on focus lost",
})

vim.api.nvim_create_autocmd("BufLeave", {
	group = "autosave",
	pattern = "*",
	command = "wall!",
	desc = "Save all buffers on buffer leave",
})

-- Start insert mode in terminal buffers
vim.api.nvim_create_augroup("terminal", { clear = true })
vim.api.nvim_create_autocmd({ "BufWinEnter", "WinEnter" }, {
	group = "terminal",
	pattern = "term://*",
	command = "startinsert",
	desc = "Enter insert mode in terminal buffers",
})

local COLORSCHEME = {
	"ellisonleao/gruvbox.nvim",
	lazy = false,
	priority = 1000,
	config = function()
		require("gruvbox").setup({
			terminal_colors = true,
			underline = false,
			bold = false,
			italic = {
				strings = false,
			},
			inverse = true, -- invert background for search, diffs, statuslines and errors
			contrast = "soft",
			palette_overrides = {},
			overrides = {
				["@function"] = { fg = "#ff9900" },
				["@function.method"] = { fg = "#ff9900" },
				["@function.call"] = { fg = "#ff9900" },
				["@constant.macro"] = { fg = "#fabd2f" },
				["@property"] = { fg = "#ebdbb2" },
			},
			dim_inactive = false,
			transparent_mode = false,
		})

		vim.cmd([[colorscheme gruvbox]])
	end,
}

local FORMATTER = {
	"stevearc/conform.nvim",
	lazy = true,

	config = function()
		require("conform").setup({
			formatters_by_ft = {
				lua = { "stylua" },
				python = { "black" },
				rust = { "rustfmt" },
				go = { "goimports", "gofmt" },
				javascript = { "biome", "prettier" },
				svelte = { "biome", "prettier" },
				sql = { "sql-formatter" },
				md = { "mdformat" },
			},
		})
	end,
	keys = function()
		vim.keymap.set("n", "F", ":Format<CR>", { noremap = true })
	end,
}

local TELESCOPE = {
	"nvim-telescope/telescope.nvim",
	branch = "0.1.x",
	dependencies = { "nvim-lua/plenary.nvim", "natecraddock/telescope-zf-native.nvim" },
	lazy = true,
	opts = {
		--		defaults = {
		--			layout_config = {
		--				height = 0.5,
		--				preview_cutoff = 200,
		--				prompt_position = "bottom",
		--				width = 0.5,
		--			},
		--		},

		pickers = {
			find_files = {
				hidden = true,
				file_ignore_patterns = {
					"node_modules",
					".git",
					".venv",
					".svelte-kit",
					"dist",
					"build",
					"target",
				},
			},
		},
	},

	keys = function()
		local telescope = require("telescope.builtin")
		vim.keymap.set("n", "<leader>p", telescope.find_files, {})

		vim.keymap.set("n", "<leader>f", function()
			telescope.grep_string({ search = vim.fn.input("grep > ") })
		end)
		vim.keymap.set("n", "gr", telescope.lsp_references, {})
		vim.keymap.set("n", "<leader>bs", telescope.buffers, {})

		require("telescope").load_extension("zf-native")
	end,
}

local function nvimtree_on_attach(bufnr)
	local nvimtree = require("nvim-tree.api")
	local function opts(desc)
		return { desc = "nvim-tree: " .. desc, buffer = bufnr, noremap = true, silent = true, nowait = true }
	end
	nvimtree.config.mappings.default_on_attach(bufnr)
	vim.keymap.del("n", "K", { buffer = bufnr })
	vim.keymap.del("n", "r", { buffer = bufnr })
	vim.keymap.del("n", "<C-e>", { buffer = bufnr })
	vim.keymap.del("n", "o", { buffer = bufnr })
	vim.keymap.del("n", "d", { buffer = bufnr })
	vim.keymap.del("n", "Y", { buffer = bufnr })

	vim.keymap.set("n", "K", nvimtree.node.show_info_popup, opts("Info"))
	vim.keymap.set("n", "r", nvimtree.fs.rename_full, opts("Rename"))
	vim.keymap.set("n", "o", nvimtree.tree.change_root_to_node, opts("CD"))
	vim.keymap.set("n", "d", nvimtree.fs.trash, opts("Trash file"))
	vim.keymap.set("n", "Y", nvimtree.fs.copy.absolute_path, opts("Info"))
end

local NVIMTREE = {
	"nvim-tree/nvim-tree.lua",
	lazy = true,
	dependencies = { "nvim-tree/nvim-web-devicons" },
	config = function()
		require("nvim-tree").setup({
			on_attach = nvimtree_on_attach,
			sort = {
				sorter = "case_sensitive",
			},
			view = {
				width = 30,
			},
			filters = {
				custom = { "^\\.git" },
				exclude = { ".gitignore" },
			},
		})
	end,
	cmd = { "NvimTreeOpen" },
	keys = function()
		vim.keymap.set("n", "<leader>e", ":NvimTreeOpen<CR>", { noremap = true, silent = true })
	end,
}
local TREESITTER = {
	"nvim-treesitter/nvim-treesitter",
	event = { "BufReadPost", "BufNewFile", "CmdlineEnter" },
	cmd = { "TSUpdateSync", "TSUpdate", "TSInstall" },
	config = function()
		require("nvim-treesitter.configs").setup({
			ensure_installed = {
				"c",
				"lua",
				"python",
				"javascript",
				"typescript",
				"php",
				"go",
				"html",
				"css",
				"svelte",
			},
			sync_install = false,
			auto_install = false,
			highlight = {
				enable = true,
				additional_vim_regex_highlighting = false,
				priority = 200, -- Increase priority (default is ~100)
			},
			indent = { enable = true },
		})
	end,
}

local MISC = {
	-- extra colorscheme
	{ "talha-akram/noctis.nvim", lazy = true, event = "CmdlineEnter" },
	{ "binhtran432k/dracula.nvim", lazy = true, event = "CmdlineEnter" },
	{ "kkkfasya/frappeless.nvim", lazy = true, event = "CmdlineEnter" },

	{ "christoomey/vim-tmux-navigator", lazy = true, event = "BufReadPost" },
	{ "AlexeySachkov/llvm-vim", lazy = true, ft = "llvm" },
	{ "https://github.com/pigpigyyy/YueScript-vim" },

	{ "mg979/vim-visual-multi", lazy = true, event = { "BufReadPost", "BufNewFile" } },
	{ "akinsho/bufferline.nvim", lazy = true, opts = {}, event = "BufReadPost" },
	{ "kkkfasya/timelapse.nvim", lazy = true, cmd = { "Timelapse" } }, -- my own plugin!!!

	{
		"windwp/nvim-autopairs",
		lazy = true,
		event = "InsertEnter",
		opts = { enable_check_bracket_line = false },
	},

	{
		"catgoose/nvim-colorizer.lua",
		lazy = true,
		event = "BufReadPre",
		ft = { "css", "javascript", "html" },
		opts = {
			filetypes = { "css", "javascript", "html" },
			tailwind = true,
			tailwind_opts = {
				update_names = "lsp",
			},
		},
	},

	{
		"folke/trouble.nvim",
		lazy = true,
		opts = {},
		cmd = { "Trouble" },
		keys = function()
			vim.keymap.set("n", "<leader>xx", "<cmd>Trouble diagnostics toggle<cr>", { noremap = true })
		end,
	},

	{
		"folke/todo-comments.nvim",
		event = "BufReadPost",
		opts = { highlight = { multiline = false } },
	},

	{
		"famiu/bufdelete.nvim",
		lazy = true,
		cmd = { "Bdelete" },
		keys = function()
			vim.keymap.set("n", "<leader>qq", "<cmd>Bdelete<CR>", { noremap = true, silent = true })
		end,
	},
	{
		"ej-shafran/compile-mode.nvim",
		lazy = true,
		cmd = { "Compile", "Recompile" },
		keys = function()
			vim.keymap.set("n", "<leader>C", ":w | :Compile<CR>", { noremap = true })
			vim.keymap.set("n", "<leader>cc", ":w | :Recompile<CR>", { noremap = true })
		end,
	},

	{
		"rmagatti/goto-preview",
		lazy = true,
		opts = {
			post_open_hook = function(buf, win)
				vim.keymap.set("n", "q", "<cmd>lua require('goto-preview').close_all_win()<CR>", { noremap = true })
			end,
		},
		keys = function()
			vim.keymap.set(
				"n",
				"gp",
				"<cmd>lua require('goto-preview').goto_preview_definition()<CR>",
				{ noremap = true }
			)
		end,
	},

	{
		"nvim-lualine/lualine.nvim",
		lazy = false,
		opts = { theme = "auto" },
		config = function(_, opts)
			require("lualine").setup({
				options = opts,
			})
		end,
	},
	{
		"numToStr/Comment.nvim",
		event = "BufReadPre",
		opts = {
			opleader = {
				line = "<C-_>",
			},
			toggler = {
				line = "<C-_>",
			},
		},
	},

	{
		"folke/snacks.nvim",
		opts = {
			quickfile = { enabled = true },
			image = { enabled = true },
			indent = {
				priority = 1,
				enabled = true,
				char = "│",
				only_scope = false,
				only_current = false,
				hl = "SnacksIndent",
				animate = { enabled = false }, -- fuck aniamation bro
			},
		},
	},

	{
		"iamcco/markdown-preview.nvim",
		cmd = { "MarkdownPreviewToggle", "MarkdownPreview", "MarkdownPreviewStop" },
		build = "cd app && npm install",
		init = function()
			vim.g.mkdp_filetypes = { "markdown" }
		end,
		ft = { "markdown" },
	},
}

local VSCODE = {
	"vscode-neovim/vscode-multi-cursor.nvim",
	event = "VeryLazy",
	cond = not not vim.g.vscode,
	opts = {},
	config = function()
		vim.keymap.set({ "n", "x", "i" }, "<C-d>", function()
			require("vscode-multi-cursor").addSelectionToNextFindMatch()
		end)
		require("vscode-multi-cursor").setup()
	end,
}

local GITSIGNS = {
	"lewis6991/gitsigns.nvim",
	event = "BufReadPre",
	opts = {
		on_attach = function(buffer)
			local gs = package.loaded.gitsigns

			local function map(mode, l, r, desc)
				vim.keymap.set(mode, l, r, { buffer = buffer, desc = desc })
			end

			map("n", "]g", function()
				if vim.wo.diff then
					vim.cmd.normal({ "]c", bang = true })
				else
					gs.nav_hunk("next")
				end
			end, "Next Hunk")

			map("n", "[g", function()
				if vim.wo.diff then
					vim.cmd.normal({ "[c", bang = true })
				else
					gs.nav_hunk("prev")
				end
			end, "Prev Hunk")

			map("n", "]H", function()
				gs.nav_hunk("last")
			end, "Last Hunk")
			map("n", "[H", function()
				gs.nav_hunk("first")
			end, "First Hunk")

			map("v", "ga", function()
				gs.stage_hunk({ vim.fn.line("."), vim.fn.line("v") })
			end)

			map("v", "gr", function()
				gs.reset_hunk({ vim.fn.line("."), vim.fn.line("v") })
			end)

			map("n", "<leader>ga", gs.stage_buffer, "Stage Buffer")
			map("n", "<leader>gR", gs.reset_buffer, "Reset Buffer")
			map("n", "<leader>gh", gs.preview_hunk_inline, "Preview Hunk Inline")

			map("n", "<leader>gb", function()
				gs.blame()
			end, "Blame Buffer")

			map("n", "<leader>ghd", gs.diffthis, "Diff This")
			map("n", "<leader>ghD", function()
				gs.diffthis("~")
			end, "Diff This ~")

			map("n", "<leader>td", gs.toggle_deleted)
			map("n", "<leader>tw", gs.toggle_word_diff)
		end,
	},
}

local AUTOCOMPLETE = {
	"Saghen/blink.cmp",
	version = "1.*",
	event = { "BufReadPost", "CmdlineEnter" },
	dependencies = {
		{ "rafamadriz/friendly-snippets" },
	},
	opts = {
		fuzzy = {
			implementation = "prefer_rust",
			sorts = {
				"exact",
				"score",
				"sort_text",
			},
		},

		completion = {
			keyword = { range = "full" },
			accept = { auto_brackets = { enabled = true } },
			list = { selection = { preselect = true, auto_insert = true } },

			menu = {
				auto_show = true,
				draw = {
					columns = {
						{ "label", "label_description", gap = 1 },
						{ "kind_icon", "kind" },
					},
					treesitter = { "lsp" },
				},
			},

			documentation = { auto_show = true, auto_show_delay_ms = 100 },

			-- Display a preview of the selected item on the current line
			ghost_text = { enabled = true },
		},

		sources = {
			default = { "lsp", "path", "buffer" },
		},

		signature = { enabled = true },

		keymap = {
			["<CR>"] = { "accept", "fallback" },
			["<Tab>"] = { "select_next", "fallback" },
			["<Down>"] = { "select_next", "fallback" },
			["<Up>"] = { "select_prev", "fallback" },
			["<S-Tab>"] = { "snippet_backward", "fallback" },
			["<C-p>"] = { "select_prev", "fallback_to_mappings" },
			["<C-n>"] = { "select_next", "fallback_to_mappings" },
			["<C-b>"] = { "scroll_documentation_up", "fallback" },
			["<C-f>"] = { "scroll_documentation_down", "fallback" },

			["<C-k>"] = { "show_signature", "hide_signature", "fallback" },
		},

		cmdline = {
			keymap = {
				["<Tab>"] = { "show", "accept" },
				["<Down>"] = { "select_next", "fallback" },
				["<Up>"] = { "select_prev", "fallback" },
			},
			completion = { menu = { auto_show = true } },
		},
	},
}

local LSPCONFIG = {
	"neovim/nvim-lspconfig",
	lazy = true,
	event = "BufRead",
	priority = 1001,
	dependencies = {
		{ "mason-org/mason.nvim", opts = {}, event = "CmdlineEnter" },
		{
			"mason-org/mason-lspconfig.nvim",
			opts = {
				ensure_installed = {
					"lua_ls",
					"clangd",
					"vtsls",
					"pyright",
					"gopls",
					"html",
					"cssls",
					"emmet_language_server",
					"tailwindcss",
					"svelte",
					"rust_analyzer",
				},
			},
		},
	},
	opts = {
		servers = {
			svelte = {},
			lua_ls = {},
			tailwindcss = {},
			gopls = {},
			clangd = {},
			pyright = {
				root_dir = function(_)
					return vim.loop.cwd()
				end,
			},
			vtsls = {},
			html = {
				format = {
					templating = true,
					wrapLineLength = 120,
					wrapAttributes = "auto",
				},
				hover = {
					documentation = true,
					references = true,
				},
			},
			cssls = {},
			emmet_language_server = {},
			rust_analyzer = { check_on_save = false },
			bacon_ls = {},
			intelephense = {
				check_on_save = false,
				root_dir = function(_)
					return vim.loop.cwd()
				end,
			},
		},
	},

	config = function(_, opts)
		local lspconfig_defaults = require("lspconfig").util.default_config
		lspconfig_defaults.capabilities =
			vim.tbl_deep_extend("force", lspconfig_defaults.capabilities, require("blink.cmp").get_lsp_capabilities())
	end,

	-- im not very fond of the way lazy setup lazy-keymap but thankfully folke allows function
	-- so i can set it up the normal way
	keys = function()
		vim.keymap.set("n", "gd", vim.lsp.buf.definition, {})
		vim.keymap.set("n", "gi", vim.lsp.buf.implementation, {})
		vim.keymap.set("n", "K", vim.lsp.buf.hover, {})
		vim.keymap.set("n", "E", vim.diagnostic.open_float, {})
		vim.keymap.set("n", "ge", vim.diagnostic.goto_next, {})
		vim.keymap.set("n", "gE", vim.diagnostic.goto_prev, {})
		vim.keymap.set("n", "<leader>r", vim.lsp.buf.rename, {})
		vim.keymap.set("n", "<leader>qf", function()
			vim.lsp.buf.code_action({
				filter = function(a)
					return a.isPreferred
				end,
				apply = true,
			})
		end, { noremap = true, silent = true })
	end,
}

-- require("lspconfig").phpactor.setup({
-- 	capabilities = capabilities,
--     init_options = {
--         ["language_server_phpstan.enabled"] = true,
--         ["language_server_psalm.enabled"] = true,
--     },
--     root_dir = function(_)
--         return vim.loop.cwd()
--     end,
-- })

-- General Mappings
vim.keymap.set("n", "<Esc>", "<cmd>nohlsearch<CR>")
vim.keymap.set("n", "<C-Left>", ":vertical resize +3<CR>", { silent = true, desc = "Increase window width" })
vim.keymap.set("n", "<C-Right>", ":vertical resize -3<CR>", { silent = true, desc = "Decrease window width" })
vim.keymap.set("n", "<C-Up>", ":horizontal resize +3<CR>", { silent = true, desc = "Increase window width" })
vim.keymap.set("n", "<C-Down>", ":horizontal resize -3<CR>", { silent = true, desc = "Decrease window width" })

vim.keymap.set("n", "<leader>bb", ":b#<CR>", {})
vim.keymap.set("n", "<leader>bn", ":bn<CR>", {})

vim.keymap.set("n", "<C-k>", ":wincmd k<CR>", { silent = true, desc = "Move to window above" })
vim.keymap.set("n", "<C-j>", ":wincmd j<CR>", { silent = true, desc = "Move to window below" })
vim.keymap.set("n", "<C-h>", ":wincmd h<CR>", { silent = true, desc = "Move to window left" })
vim.keymap.set("n", "<C-l>", ":wincmd l<CR>", { silent = true, desc = "Move to window right" })

-- USER COMMANDS
vim.api.nvim_create_user_command("ListSymbols", function()
	vim.cmd(":lua require'telescope.builtin'.treesitter{}")
end, {})

vim.api.nvim_create_user_command("Format", function(args)
	local range = nil
	if args.count ~= -1 then
		local end_line = vim.api.nvim_buf_get_lines(0, args.line2 - 1, args.line2, true)[1]
		range = {
			start = { args.line1, 0 },
			["end"] = { args.line2, end_line:len() },
		}
	end
	require("conform").format({ async = true, lsp_format = "never", range = range })
end, { range = true })

vim.api.nvim_create_user_command("VirtualTextToggle", function()
	local cfg = vim.diagnostic.config()

	local status = (not cfg.virtual_text) and "enabled" or "disabled"
	print("Diagnostic virtual text is " .. status)
end, {})

vim.api.nvim_create_user_command("DiagnosticsToggle", function()
	local diagnostic = vim.diagnostic.is_enabled()
	vim.diagnostic.enable(not diagnostic)

	local status = (not diagnostic) and "enabled" or "disabled"
	print("Diagnostic is " .. status)
end, {})

require("lazy").setup({
	rocks = {
		enabled = false,
	},
	spec = {
		COLORSCHEME,
		FORMATTER,
		NVIMTREE,
		GITSIGNS,
		LSPCONFIG,
		AUTOCOMPLETE,
		TELESCOPE,
		TREESITTER,
		VSCODE,
		MISC,
	},
	install = { colorscheme = { "gruvbox" } },
	checker = { enabled = false },
})

vim.diagnostic.config({
	virtual_text = false,
})
                &args.regrab,
                &args.include,
                &args.exclude,
                !args.no_line_numbers,
                args.octal_perms,
                args.owner_type,
                &args.time_format,
                &timezone,
                &args.custom_format,
                args.deep,
                args.toggle_clock,
            );

            if let Some(order) = sort_order {
                sort_files(&mut files, &order);
            }

            if !args.no_line_numbers {
                add_line_numbers(&mut files);
            }

            if !tty_available {
                print_names_only(&files);
                exit(0);
            }

            if args.json {
                println!("{}", serde_json::to_string_pretty(&files).unwrap());
            } else if args.regrab.is_none() {
                print_table_from_files(&files, &args.grab, !args.no_line_numbers);
            } else {
                print_table_from_files(&files, &args.regrab, !args.no_line_numbers);
            }

            if let Some(export_path) = &args.json_export {
                if let Err(e) = export_json(&files, export_path) {
                    eprintln!("{}: {}", "Error writing JSON file".red(), e);
                    exit(1);
                }
                println!("{} {}", "JSON exported to:".green(), export_path.display());
            }
        } else {
            eprintln!("{}", "Path does not exist.".red());
            exit(1);
        }
    } else {
        eprintln!("{}", "Error checking path.".red());
        exit(1);
    }
}

fn add_line_numbers(files: &mut [FileEntry]) {
    for (index, file) in files.iter_mut().enumerate() {
        file.line_number = (index + 1).to_string();
    }
}

fn print_names_only(files: &[FileEntry]) {
    for file in files {
        println!("{}", file.name);
    }
}
