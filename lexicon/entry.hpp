
#ifndef ENTRY_HPP
#define ENTRY_HPP

#include <string>

struct Entry {

  const std::string title;
  const std::string pos;
  const std::string content;

  Entry(const std::string title, const std::string pos,
        const std::string content)
      : title(title), pos(pos), content(content){};

  Entry(const std::string title, const std::string content)
      : title(title), pos(""), content(content){};
};

#endif
