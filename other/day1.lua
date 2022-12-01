function AdventOfCode1()
  local a = (vim.api.nvim_buf_get_lines(vim.api.nvim_get_current_buf(), 0, -1, true))

  local elves = {}
  local sum = 0
  for _, v in pairs(a) do
    if v == "" then
      table.insert(elves, sum)
      sum = 0
    else
      sum = sum + tonumber(v)
    end
  end

  table.sort(elves)

  local answer1 = elves[#elves]
  local answer2 = elves[#elves] + elves[#elves-1] + elves[#elves-2]

  vim.api.nvim_buf_set_lines(vim.api.nvim_get_current_buf(), 1, -1, false, {})
  vim.api.nvim_buf_set_lines(vim.api.nvim_get_current_buf(), 0, 1, false, {"part 1: " .. answer1})
  vim.api.nvim_buf_set_lines(vim.api.nvim_get_current_buf(), 1, 2, false, {"part 2: " .. answer2})
end
